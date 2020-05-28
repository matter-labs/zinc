pub mod arguments;

use std::process::exit;

use structopt::StructOpt;

use self::arguments::Arguments;
use self::arguments::Command;

fn main() {
    let arguments: Arguments = Arguments::from_args();

    let result = match arguments.command {
        Command::GenKey(c) => c.execute(),
        Command::PubKey(c) => c.execute(),
        Command::Sign(c) => c.execute(),
    };

    if let Err(error) = result {
        eprintln!("{}", error);
        exit(1);
    }
}
