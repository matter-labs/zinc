//!
//! The Zinc virtual machine `test` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Application as BuildApplication;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "test", about = "Executes a unit test")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary")]
    pub binary_path: PathBuf,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let application = BuildApplication::try_from_slice(bytes.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        let status = match application {
            BuildApplication::Circuit(circuit) => CircuitFacade::new(circuit).test::<Bn256>()?,
            BuildApplication::Contract(contract) => {
                ContractFacade::new(contract).test::<Bn256>()?
            }
        };

        Ok(status as i32)
    }
}
