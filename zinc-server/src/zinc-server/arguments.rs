//!
//! The Zinc server arguments.
//!

use structopt::StructOpt;

///
/// The Zinc server arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = zinc_const::app_name::ZINC_SERVER,
    about = "The Zinc server"
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

    /// The MongoDB server host.
    #[structopt(
        long = "mongodb-host",
        help = "The MongoDB server host",
        default_value = zinc_const::mongodb::HOST,
    )]
    pub mongodb_host: String,

    /// The MongoDB server port.
    #[structopt(long = "mongodb-port", help = "The MongoDB server port")]
    pub mongodb_port: Option<u16>,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
