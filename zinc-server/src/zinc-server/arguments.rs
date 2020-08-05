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

    /// The PostgreSQL database host.
    #[structopt(
        long = "postgres-host",
        help = "The PostgreSQL database host",
        default_value = zinc_const::postgresql::HOST,
    )]
    pub postgresql_host: String,

    /// The PostgreSQL database port.
    #[structopt(long = "postgres-port", help = "The PostgreSQL database port")]
    pub postgresql_port: Option<u16>,

    /// The PostgreSQL user name.
    #[structopt(
        long = "postgres-user",
        help = "The PostgreSQL user name",
        default_value = zinc_const::postgresql::USER,
    )]
    pub postgresql_user: String,

    /// The PostgreSQL user password.
    #[structopt(
        long = "postgres-password",
        help = "The PostgreSQL user password",
        default_value = zinc_const::postgresql::PASSWORD,
    )]
    pub postgresql_password: String,

    /// The PostgreSQL database name.
    #[structopt(
        long = "postgres-database",
        help = "The PostgreSQL database name",
        default_value = zinc_const::postgresql::DATABASE,
    )]
    pub postgresql_database: String,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
