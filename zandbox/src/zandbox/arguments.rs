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
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The HTTP server port.
    #[structopt(long = "http-port", help = "The HTTP server port")]
    pub http_port: Option<u16>,

    /// The PostgreSQL connection string.
    #[structopt(long = "postgresql", help = "The PostgreSQL connection string")]
    pub postgresql_uri: String,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
