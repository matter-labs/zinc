//!
//! The ordinar integration test project.
//!

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Context;
use colored::Colorize;
use tokio::runtime::Runtime;

use crate::ordinar::action::call::Call as CallAction;
use crate::ordinar::action::publish::Publish as PublishAction;
use crate::ordinar::action::query::Query as QueryAction;
use crate::ordinar::action::Action;
use crate::summary::Summary;

///
/// The ordinar integration test project.
///
#[derive(Debug)]
pub struct Project {
    /// If zero, does not print the successful tests.
    pub verbosity: usize,
    /// The project path.
    pub path: PathBuf,
    /// The asynchronous runtime.
    pub runtime: Runtime,
    /// The published instance addresses.
    pub instance_addresses: HashMap<String, zksync_types::Address>,
}

impl Project {
    const INSTANCE_ADDRESSES_INITIAL_CAPACITY: usize = 4;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(verbosity: usize, path: PathBuf) -> Self {
        Self {
            verbosity,
            path,
            runtime: tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME),
            instance_addresses: HashMap::with_capacity(Self::INSTANCE_ADDRESSES_INITIAL_CAPACITY),
        }
    }

    ///
    /// Runs the test project and writes its results to `summary`.
    ///
    pub fn run(mut self, summary: Arc<Mutex<Summary>>) -> anyhow::Result<()> {
        if self.build(summary.clone()).is_err() {
            self.clean()?;
            return Ok(());
        }

        if self.test(summary.clone()).is_err() {
            self.clean()?;
            return Ok(());
        }

        let scenarios = match self.get_scenarios() {
            Ok(scenarios) => scenarios,
            Err(error) => {
                println!(
                    "[INTEGRATION] {} {} (scenarios): {:?}",
                    "INVALID".red(),
                    self.path.to_string_lossy(),
                    error,
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .invalid += 1;
                return Ok(());
            }
        };
        for scenario in scenarios.into_iter() {
            for manifest_path in self.get_dependency_paths()?.into_iter() {
                if self.upload(summary.clone(), manifest_path).is_err() {
                    self.clean()?;
                    return Ok(());
                }
            }

            for action in scenario.into_iter() {
                match action {
                    Action::Publish(inner) => {
                        if self.publish(summary.clone(), inner).is_err() {
                            self.clean()?;
                            return Ok(());
                        }
                    }
                    Action::Query(inner) => {
                        if self.query(summary.clone(), inner).is_err() {
                            self.clean()?;
                            return Ok(());
                        }
                    }
                    Action::Call(inner) => {
                        if self.call(summary.clone(), inner).is_err() {
                            self.clean()?;
                            return Ok(());
                        }
                    }
                }
            }
        }

        if self.verbosity > 0 {
            println!(
                "[INTEGRATION] {} {}",
                "PASSED".green(),
                self.path.to_string_lossy(),
            );
        }
        summary
            .lock()
            .expect(zinc_const::panic::SYNCHRONIZATION)
            .passed += 1;

        self.clean()?;

        Ok(())
    }

    ///
    /// Builds the test project.
    ///
    fn build(&mut self, summary: Arc<Mutex<Summary>>) -> anyhow::Result<()> {
        if let Err(error) = self.runtime.block_on(
            zargo::BuildCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                self.path.clone(),
                false,
                Some(zksync::Network::Localhost.to_string()),
            )
            .execute(),
        ) {
            if self.path.to_string_lossy().contains("error") {
                if self.verbosity > 0 {
                    println!(
                        "[INTEGRATION] {} {} (panicked)",
                        "PASSED".green(),
                        self.path.to_string_lossy(),
                    );
                }
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .passed += 1;
            } else {
                println!(
                    "[INTEGRATION] {} {}: {:?}",
                    "INVALID".red(),
                    self.path.to_string_lossy(),
                    error,
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .invalid += 1;
            }

            return Err(error);
        }

        Ok(())
    }

    ///
    /// Runs the project integration tests.
    ///
    fn test(&mut self, summary: Arc<Mutex<Summary>>) -> anyhow::Result<()> {
        if let Err(error) = self.runtime.block_on(
            zargo::TestCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                self.path.clone(),
                Some(zksync::Network::Localhost.to_string()),
            )
            .execute(),
        ) {
            println!(
                "[INTEGRATION] {} {} (unit test failure): {:?}",
                "FAILED".bright_red(),
                self.path.to_string_lossy(),
                error,
            );
            summary
                .lock()
                .expect(zinc_const::panic::SYNCHRONIZATION)
                .failed += 1;
            return Err(error);
        }

        Ok(())
    }

    ///
    /// Uploads a test project dependency.
    ///
    fn upload(
        &mut self,
        summary: Arc<Mutex<Summary>>,
        manifest_path: PathBuf,
    ) -> anyhow::Result<()> {
        if let Err(error) = self.runtime.block_on(
            zargo::UploadCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                manifest_path.clone(),
                Some(zksync::Network::Localhost.to_string()),
            )
            .execute(),
        ) {
            println!(
                "[INTEGRATION] {} {} (dependency upload): {:?}",
                "INVALID".red(),
                manifest_path.to_string_lossy(),
                error,
            );
            summary
                .lock()
                .expect(zinc_const::panic::SYNCHRONIZATION)
                .invalid += 1;
            return Err(error);
        }

        Ok(())
    }

    ///
    /// Publishes a contract instance of the test project.
    ///
    fn publish(
        &mut self,
        summary: Arc<Mutex<Summary>>,
        action: PublishAction,
    ) -> anyhow::Result<()> {
        self.copy_scenario_input(summary.clone(), action.input_path)?;

        match self.runtime.block_on(
            zargo::PublishCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                self.path.clone(),
                action.instance.clone(),
                Some(zksync::Network::Localhost.to_string()),
                None,
            )
            .execute(),
        ) {
            Ok(data) => {
                self.instance_addresses
                    .insert(action.instance, data.address);
                Ok(())
            }
            Err(error) => {
                println!(
                    "[INTEGRATION] {} {} (publish failure): {:?}",
                    "FAILED".bright_red(),
                    self.path.to_string_lossy(),
                    error,
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .failed += 1;
                Err(error)
            }
        }
    }

    ///
    /// Queries a contract instance of the test project.
    ///
    fn query(&mut self, summary: Arc<Mutex<Summary>>, action: QueryAction) -> anyhow::Result<()> {
        self.copy_scenario_input(summary.clone(), action.input_path)?;
        let input_destination = self.input_destination();

        let address = match self.instance_addresses.get(&action.instance).cloned() {
            Some(address) => serde_json::to_string(&address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", ""),
            None => {
                println!(
                    "[INTEGRATION] {} {} (instance address missing)",
                    "INVALID".red(),
                    self.path.to_string_lossy(),
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .invalid += 1;
                anyhow::bail!("Instance `{}` address is missing", action.instance);
            }
        };
        if let Err(error) = self
            .set_input_address(
                &input_destination,
                address.as_str(),
                action.method.is_none(),
            )
            .with_context(|| input_destination.to_string_lossy().to_string())
        {
            println!(
                "[INTEGRATION] {} {} (input file address setting): {:?}",
                "INVALID".red(),
                self.path.to_string_lossy(),
                error,
            );
            summary
                .lock()
                .expect(zinc_const::panic::SYNCHRONIZATION)
                .invalid += 1;
            anyhow::bail!("Input file `{}` address setting", action.instance);
        }

        match self.runtime.block_on(
            zargo::QueryCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                self.path.clone(),
                Some(zksync::Network::Localhost.to_string()),
                address,
                action.method,
            )
            .execute(),
        ) {
            Ok(mut output) => {
                if let Err(error) = self.remove_address(&mut output) {
                    println!(
                        "[INTEGRATION] {} (storage output address setting): {:?}",
                        "INVALID".red(),
                        error,
                    );
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .invalid += 1;
                    anyhow::bail!(
                        "Instance `{}` storage output address setting",
                        action.instance
                    );
                }

                if output != action.expect {
                    println!(
                        "[INTEGRATION] {} {} (query failure): (expected `{}`, found `{}`)",
                        "FAILED".bright_red(),
                        self.path.to_string_lossy(),
                        action.expect,
                        output,
                    );
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .failed += 1;
                    anyhow::bail!("Query output does not match the expected");
                }

                Ok(())
            }
            Err(error) => {
                println!(
                    "[INTEGRATION] {} {} (query failure): {:?}",
                    "FAILED".bright_red(),
                    self.path.to_string_lossy(),
                    error,
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .failed += 1;
                Err(error)
            }
        }
    }

    ///
    /// Calls a method of a contract instance of the test project.
    ///
    fn call(&mut self, summary: Arc<Mutex<Summary>>, action: CallAction) -> anyhow::Result<()> {
        self.copy_scenario_input(summary.clone(), action.input_path)?;
        let input_destination = self.input_destination();

        let address = match self.instance_addresses.get(&action.instance).cloned() {
            Some(address) => serde_json::to_string(&address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", ""),
            None => {
                println!(
                    "[INTEGRATION] {} {} (instance address missing)",
                    "INVALID".red(),
                    self.path.to_string_lossy(),
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .invalid += 1;
                anyhow::bail!("Instance `{}` address is missing", action.instance);
            }
        };
        if let Err(error) = self
            .set_input_address(&input_destination, address.as_str(), false)
            .with_context(|| input_destination.to_string_lossy().to_string())
        {
            println!(
                "[INTEGRATION] {} {} (input file address setting): {:?}",
                "INVALID".red(),
                self.path.to_string_lossy(),
                error,
            );
            summary
                .lock()
                .expect(zinc_const::panic::SYNCHRONIZATION)
                .invalid += 1;
            anyhow::bail!("Input file `{}` address setting", action.instance);
        }

        match self.runtime.block_on(
            zargo::CallCommand::new(
                self.verbosity,
                self.verbosity <= 1,
                self.path.clone(),
                Some(zksync::Network::Localhost.to_string()),
                address,
                action.method,
            )
            .execute(),
        ) {
            Ok(output) => {
                if output != action.expect {
                    println!(
                        "[INTEGRATION] {} {} (call failure): (expected `{}`, found `{}`)",
                        "FAILED".bright_red(),
                        self.path.to_string_lossy(),
                        action.expect,
                        output,
                    );
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .failed += 1;
                    anyhow::bail!("Call output does not match the expected");
                }

                Ok(())
            }
            Err(error) => {
                println!(
                    "[INTEGRATION] {} {} (call failure): {:?}",
                    "FAILED".bright_red(),
                    self.path.to_string_lossy(),
                    error,
                );
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .failed += 1;
                Err(error)
            }
        }
    }

    ///
    /// Cleans up the test project.
    ///
    fn clean(&self) -> anyhow::Result<()> {
        zargo::CleanCommand::new(
            self.verbosity,
            self.verbosity <= 1,
            self.path.clone(),
            false,
        )
        .execute()
        .with_context(|| self.path.to_string_lossy().to_string())?;

        Ok(())
    }

    ///
    /// Sets the contract address in the input JSON template file.
    ///
    fn set_input_address(
        &mut self,
        path: &PathBuf,
        address: &str,
        is_storage_query: bool,
    ) -> anyhow::Result<()> {
        let mut json_data = {
            let mut input_file =
                fs::File::open(path).with_context(|| path.to_string_lossy().to_string())?;
            let input_size = fs::metadata(path)
                .with_context(|| path.to_string_lossy().to_string())?
                .len() as usize;
            let mut input_str = String::with_capacity(input_size);
            input_file
                .read_to_string(&mut input_str)
                .with_context(|| path.to_string_lossy().to_string())?;
            let input: serde_json::Value = serde_json::from_str(input_str.as_str())
                .with_context(|| path.to_string_lossy().to_string())?;
            input
        };

        *json_data
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("JSON data is invalid"))?
            .get_mut("storage")
            .ok_or_else(|| anyhow::anyhow!("Missing `storage` section"))?
            .as_array_mut()
            .ok_or_else(|| anyhow::anyhow!("The `storage` section is not an array"))?
            .get_mut(0)
            .ok_or_else(|| anyhow::anyhow!("The `storage` first element is missing"))? =
            serde_json::Value::String(address.to_owned());
        if !is_storage_query {
            *json_data
                .as_object_mut()
                .ok_or_else(|| anyhow::anyhow!("JSON data is invalid"))?
                .get_mut("msg")
                .ok_or_else(|| anyhow::anyhow!("Missing `msg` section"))?
                .as_object_mut()
                .ok_or_else(|| anyhow::anyhow!("The `msg` section is not an object"))?
                .get_mut("recipient")
                .ok_or_else(|| anyhow::anyhow!("Missing `msg.recipient` field"))? =
                serde_json::Value::String(address.to_owned());
        }

        let mut output_file =
            fs::File::create(path).with_context(|| path.to_string_lossy().to_string())?;
        let output_str =
            serde_json::to_string_pretty(&json_data).expect(zinc_const::panic::DATA_CONVERSION);
        output_file.write_all(output_str.as_bytes())?;

        Ok(())
    }

    ///
    /// Removes the address from the received JSON.
    ///
    fn remove_address(&self, found: &mut serde_json::Value) -> anyhow::Result<()> {
        let json_data = found
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("JSON data is invalid"))?;
        json_data.remove("address");

        Ok(())
    }

    ///
    /// Gets the dependency manifest paths.
    ///
    fn get_dependency_paths(&mut self) -> anyhow::Result<Vec<PathBuf>> {
        let mut paths = Vec::new();

        let mut dependencies_directory_path = self.path.clone();
        dependencies_directory_path.push(zinc_const::directory::TARGET_DEPS);

        if !dependencies_directory_path.exists() {
            return Ok(vec![]);
        }

        for entry in fs::read_dir(dependencies_directory_path)?.into_iter() {
            let entry = entry.expect(zinc_const::panic::DATA_CONVERSION);
            let path = entry.path();
            let entry_type = entry.file_type().expect(zinc_const::panic::DATA_CONVERSION);

            if entry_type.is_dir() {
                paths.push(path);
            }
        }

        Ok(paths)
    }

    ///
    /// Gets the custom test scenarios.
    ///
    fn get_scenarios(&mut self) -> anyhow::Result<Vec<Vec<Action>>> {
        let mut scenarios = Vec::new();

        let mut scenarios_directory_path = self.path.clone();
        scenarios_directory_path.push(zinc_const::directory::SCENARIOS);

        if !scenarios_directory_path.exists() {
            return Ok(vec![]);
        }

        for entry in fs::read_dir(scenarios_directory_path)?.into_iter() {
            let entry = entry.expect(zinc_const::panic::DATA_CONVERSION);
            let path = entry.path();
            let entry_type = entry.file_type().expect(zinc_const::panic::DATA_CONVERSION);

            let mut scenario_path = path.clone();
            if entry_type.is_dir() {
                scenario_path.push(format!(
                    "{}.{}",
                    zinc_const::file_name::SCENARIO,
                    zinc_const::extension::JSON
                ));
            } else {
                continue;
            }

            let mut scenario_file = fs::File::open(&scenario_path)
                .with_context(|| scenario_path.to_string_lossy().to_string())?;
            let scenario_size = fs::metadata(&scenario_path)
                .with_context(|| scenario_path.to_string_lossy().to_string())?
                .len() as usize;
            let mut scenario_str = String::with_capacity(scenario_size);
            scenario_file
                .read_to_string(&mut scenario_str)
                .with_context(|| scenario_path.to_string_lossy().to_string())?;
            let mut scenario: Vec<Action> = serde_json::from_str(scenario_str.as_str())
                .with_context(|| scenario_path.to_string_lossy().to_string())?;
            for action in scenario.iter_mut() {
                let mut action_input_path = path.clone();
                action_input_path.push(action.input_path());
                action.set_input_path(action_input_path);
            }

            scenarios.push(scenario);
        }

        Ok(scenarios)
    }

    ///
    /// Copy a scenario input file to the project `data` folder.
    ///
    fn copy_scenario_input(
        &self,
        summary: Arc<Mutex<Summary>>,
        source_path: PathBuf,
    ) -> anyhow::Result<()> {
        let input_destination = self.input_destination();
        if let Err(error) = fs::copy(&source_path, &input_destination)
            .with_context(|| source_path.to_string_lossy().to_string())
        {
            println!(
                "[INTEGRATION] {} {} (scenario file copying)",
                "INVALID".red(),
                source_path.to_string_lossy(),
            );
            summary
                .lock()
                .expect(zinc_const::panic::SYNCHRONIZATION)
                .invalid += 1;
            anyhow::bail!(
                "Scenario file `{}` copying: {}",
                source_path.to_string_lossy(),
                error
            );
        }

        Ok(())
    }

    ///
    /// Returns the path where the input JSON template file must be moved to.
    ///
    fn input_destination(&self) -> PathBuf {
        let mut input_destination = self.path.clone();
        input_destination.push(zinc_const::directory::DATA);
        input_destination.push(format!(
            "{}.{}",
            zinc_const::file_name::INPUT,
            zinc_const::extension::JSON
        ));
        input_destination
    }
}
