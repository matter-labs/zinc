//!
//! The Zinc virtual machine `run` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;
use zinc_mongo::Client as MongoClient;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `run` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "run", about = "Executes circuit and prints program's output")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(long = "witness", help = "The witness JSON file")]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(long = "public-data", help = "The public data JSON file")]
    pub public_data_path: PathBuf,

    /// The MongoDB server host.
    #[structopt(
        long = "mongodb-host",
        help = "The MongoDB server host",
        default_value = zinc_const::mongodb::HOST,
    )]
    pub mongodb_host: String,

    /// The MongoDB server port.
    #[structopt(long = "mongodb-port", help = "The MongoDB server port")]
    pub mongodb_port: Option<u16>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BytecodeProgram::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;
        let name = program.name();

        let input_text = fs::read_to_string(&self.witness_path)
            .error_with_path(|| self.witness_path.to_string_lossy())?;
        let json = serde_json::from_str(&input_text)?;
        let input = TemplateValue::try_from_typed_json(json, program.input())?;

        let mut runtime = zinc_mongo::Runtime::new().expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

        let mongo_client = runtime.block_on(MongoClient::new(
            self.mongodb_host,
            self.mongodb_port.unwrap_or(zinc_const::mongodb::PORT),
        ));

        let output = match program {
            BytecodeProgram::Circuit(circuit) => CircuitFacade::new(circuit).run::<Bn256>(input)?,
            BytecodeProgram::Contract(contract) => {
                let storage = runtime
                    .block_on(mongo_client.get_storage(name.as_str()))
                    .map_err(Error::MongoDb)?;
                let (output, storage) =
                    ContractFacade::new(contract).run::<Bn256>(input, Some(storage))?;
                runtime
                    .block_on(mongo_client.update_storage(name.as_str(), storage))
                    .map_err(Error::MongoDb)?;
                output
            }
        };

        let public_data_path = self.public_data_path;
        let output_json = serde_json::to_string_pretty(&output.into_json())? + "\n";
        fs::write(&public_data_path, &output_json)
            .error_with_path(|| public_data_path.to_string_lossy())?;

        print!("{}", output_json);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
