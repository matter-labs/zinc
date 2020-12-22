//!
//! The Zandbox server daemon arguments.
//!

use structopt::StructOpt;

///
/// The Zandbox server daemon arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = zinc_const::app_name::ZANDBOX,
    about = "The Zandbox server daemon"
)]
pub struct Arguments {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// The HTTP server port.
    #[structopt(short = "p", long = "http-port")]
    pub http_port: Option<u16>,

    /// The PostgreSQL connection string.
    #[structopt(short = "d", long = "postgresql")]
    pub postgresql_uri: String,

    /// The zkSync network identifier.
    #[structopt(short = "n", long = "network")]
    pub network: String,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
