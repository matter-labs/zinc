//!
//! The Jabberwocky compiler binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use compiler::Statement;

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

#[derive(Debug, Fail)]
enum FileError {
    #[fail(display = "Opening: {}", _0)]
    Opening(std::io::Error),
    #[fail(display = "Metadata: {}", _0)]
    Metadata(std::io::Error),
    #[fail(display = "Reading: {}", _0)]
    Reading(std::io::Error),
}

fn main() -> Result<(), FileError> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let mut file = File::open(&args.input).map_err(FileError::Opening)?;
    let size = file.metadata().map_err(FileError::Metadata)?.len();
    let mut code = Vec::with_capacity(size as usize);
    file.read_to_end(&mut code).map_err(FileError::Reading)?;

    let circuit = compiler::compile(code);

    if args.meta {
        let metadata = match circuit {
            Ok(ref circuit) => serde_json::to_string(circuit).expect("Serialization bug"),
            Err(ref error) => error.to_string(),
        };
        println!("{}", metadata);
    }

    match circuit {
        Ok(circuit) => {
            for statement in circuit.statements.into_iter() {
                log::debug!("{}", statement);
                if let Statement::Debug(debug) = statement {
                    match compiler::execute(debug.expression) {
                        Ok(field) => println!("{}", field.value),
                        Err(error) => log::error!("{}", error),
                    }
                }
            }
        }
        Err(error) => log::error!("{}", error),
    }

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,jabc=info");
    }
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
}
