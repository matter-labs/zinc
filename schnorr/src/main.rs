//!
//! The Schnorr signature tool binary.
//!

mod arguments;

use std::process;

use self::arguments::Arguments;
use self::arguments::Command;

fn main() {
    let arguments = Arguments::new();

    let result = match arguments.command {
        Command::GenKey(command) => command.execute(),
        Command::PubKey(command) => command.execute(),
        Command::Sign(command) => command.execute(),
    };

    if let Err(error) = result {
        eprintln!("{}", error);
        process::exit(zinc_const::exit_code::FAILURE);
    }
}
