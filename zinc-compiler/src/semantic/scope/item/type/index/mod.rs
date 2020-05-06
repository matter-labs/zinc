//!
//! The semantic analyzer scope type item index.
//!

pub mod hard;
pub mod soft;

use self::hard::Index as HardIndex;
use self::soft::Index as SoftIndex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SOFT: SoftIndex = SoftIndex::new();
    pub static ref HARD: HardIndex = HardIndex::new();
}
