//!
//! The ordinar integration test action.
//!

pub mod call;
pub mod publish;
pub mod query;

use std::path::PathBuf;

use serde::Deserialize;

use self::call::Call as CallAction;
use self::publish::Publish as PublishAction;
use self::query::Query as QueryAction;

///
/// The ordinar integration test action.
///
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum Action {
    /// The `zargo publish` command.
    Publish(PublishAction),
    /// The `zargo query` command.
    Query(QueryAction),
    /// The `zargo call` command.
    Call(CallAction),
}

impl Action {
    ///
    /// Returns the input file path.
    ///
    pub fn input_path(&self) -> PathBuf {
        match self {
            Self::Publish(inner) => inner.input_path.to_owned(),
            Self::Query(inner) => inner.input_path.to_owned(),
            Self::Call(inner) => inner.input_path.to_owned(),
        }
    }

    ///
    /// Sets the input file path. Used to replace the relative path with the absolute one.
    ///
    pub fn set_input_path(&mut self, path: PathBuf) {
        match self {
            Self::Publish(ref mut inner) => inner.input_path = path,
            Self::Query(ref mut inner) => inner.input_path = path,
            Self::Call(ref mut inner) => inner.input_path = path,
        }
    }
}
