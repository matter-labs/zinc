//!
//! The Zinc virtual machine `setup` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Program as BuildProgram;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `setup` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = "setup",
    about = "Generates a pair of proving and verifying keys"
)]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(long = "proving-key", help = "The proving key path")]
    pub proving_key_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(long = "verifying-key", help = "The verifying key path")]
    pub verifying_key_path: PathBuf,

    /// The method name to call, if the program is a contract.
    #[structopt(long = "method", help = "The method name")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BuildProgram::try_from_slice(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        let params = match program {
            BuildProgram::Circuit(circuit) => CircuitFacade::new(circuit).setup::<Bn256>()?,
            BuildProgram::Contract(contract) => {
                let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                ContractFacade::new(contract).setup::<Bn256>(method_name)?
            }
        };

        let proving_key_path = self.proving_key_path;
        let pkey_file = fs::File::create(&proving_key_path)
            .error_with_path(|| proving_key_path.to_string_lossy())?;
        params
            .write(pkey_file)
            .error_with_path(|| proving_key_path.to_string_lossy())?;

        let mut verifying_key = Vec::new();
        params.vk.write(&mut verifying_key).expect("writing to vec");
        let verifying_key_path = self.verifying_key_path;
        fs::write(&verifying_key_path, verifying_key)
            .error_with_path(|| verifying_key_path.to_string_lossy())?;

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
