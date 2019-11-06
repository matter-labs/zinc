//!
//! The bytecode viewer binary.
//!

use std::path::PathBuf;

use structopt::StructOpt;

use zrust_bytecode::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "zrustv", about = "The ZRust bytecode viewer")]
struct Arguments {
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the *.zrsb input file name"
    )]
    input: PathBuf,
}

fn main() -> Result<(), Error> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    log::info!("Input: {:?}", args.input);

    let instructions = zrust_bytecode::from_file(args.input)?;
    for instruction in instructions.into_iter() {
        log::info!("{}", instruction);
        dbg!(instruction);
    }

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "zrust_bytecode=info,zrustv=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
