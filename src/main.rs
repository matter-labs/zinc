//!
//! The Jab compiler binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabc", about = "The Jabberwocky language compiler")]
struct Arguments {
    #[structopt(
        short = "p",
        long = "profile",
        help = "Runs the profiler and print cost information"
    )]
    profile: bool,
    #[structopt(
        short = "o",
        long = "output",
        name = "OUTPUT",
        parse(from_os_str),
        default_value = "output.rs",
        help = "Specifies the output .rs file name"
    )]
    output: PathBuf,
    #[structopt(short = "m", long = "meta", help = "Generates meta info")]
    meta: bool,

    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let mut file = match File::open(&args.input) {
        Ok(file) => file,
        Err(error) => {
            error!("File {:?} opening error: {}", args.input, error);
            return;
        }
    };

    let mut code = String::with_capacity(1024);
    if let Err(error) = file.read_to_string(&mut code) {
        error!("File {:?} reading error: {}", args.input, error);
        return;
    }

    let metadata = match compiler::compile(code.to_owned()) {
        Ok(circuit) => serde_json::to_string(&circuit).expect("Serialization bug"),
        Err(error) => error.to_string(),
    };

    if args.meta {
        println!("{}", metadata);
    }
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
