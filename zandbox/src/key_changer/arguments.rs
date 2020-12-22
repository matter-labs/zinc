//!
//! The zkSync account public key changer arguments.
//!

use structopt::StructOpt;

///
/// The zkSync account public key changer arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = zinc_const::app_name::KEY_CHANGER,
    about = "The zkSync account public key changer"
)]
pub struct Arguments {
    /// The account private key.
    #[structopt(short = "k", long = "private-key")]
    pub private_key: String,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
