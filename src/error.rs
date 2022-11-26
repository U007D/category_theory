pub mod io;

use crate::shared_consts::*;
use thiserror::Error;

#[allow(dead_code)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}: ", msg::ERR_IO, 0)]
    IoError(#[from] io::Error),
}
