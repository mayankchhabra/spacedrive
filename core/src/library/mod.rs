mod loader;
mod statistics;

pub use loader::*;
pub use statistics::*;

use thiserror::Error;

use crate::sys::SysError;
use prisma;

#[derive(Error, Debug)]
pub enum LibraryError {
	#[error("Missing library")]
	LibraryNotFound,
	#[error("Database error")]
	DatabaseError(#[from] prisma::QueryError),
	#[error("System error")]
	SysError(#[from] SysError),
}
