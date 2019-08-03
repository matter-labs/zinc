//!
//! The Jab compiler binary.
//!

use failure::Fail;
use log::*;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Compiler: {}", _0)]
    Compiler(compiler::Error),
}

fn main() -> Result<(), Error> {
    init_logger();
    info!("Started");

    let code = r#"
        inputs {
            a: uint8;
            b: field;
            c: bool;
        }

        witness {
            a: uint253;
            b: field;
            c: bool;
        }
    "#;

    let circuit = compiler::compile(&code).map_err(Error::Compiler)?;
    info!("{:?}", circuit);

    info!("Ended");
    Ok(())
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
