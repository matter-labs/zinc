pub mod arguments;

use structopt::StructOpt;
use arguments::{Arguments, Command};

fn main() {
    let arguments: Arguments = Arguments::from_args();

    match arguments.command {
        Command::GenKey(c) => c.execute(),
        Command::PubKey(c) => c.execute(),
        Command::Sign(c) => c.execute(),
    }
}
