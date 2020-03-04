pub mod arguments;

use arguments::{Arguments, Command};
use structopt::StructOpt;

fn main() {
    let arguments: Arguments = Arguments::from_args();

    match arguments.command {
        Command::GenKey(c) => c.execute(),
        Command::PubKey(c) => c.execute(),
        Command::Sign(c) => c.execute(),
    }
}
