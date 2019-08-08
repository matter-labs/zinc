//!
//! The Jab compiler binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabc")]
struct Arguments {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let mut file = match File::open(&args.file) {
        Ok(file) => file,
        Err(error) => {
            error!("File {:?} opening error: {}", args.file, error);
            return;
        }
    };

    let mut code = String::with_capacity(1024);
    if let Err(error) = file.read_to_string(&mut code) {
        error!("File {:?} reading error: {}", args.file, error);
        return;
    }

    let result = match compiler::compile(code.to_owned()) {
        Ok(circuit) => serde_json::to_string(&circuit).expect("Serialization bug"),
        Err(error) => error.to_string(),
    };

    println!("{}", result);
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=trace");
    }
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
}
