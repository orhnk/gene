use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptError<'a> {
	#[error("invalid package manager: expected {expected:?} in {found:?}")]
	InvalidPackageManager {
		expected: &'a str,
		found: &'a Vec<String>,
	},

	#[error("no package manager found")]
	NoPackageManager,

	#[error("unknown option present in config file: {0}")]
	UnknownOption(String),
}
