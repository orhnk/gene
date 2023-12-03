//! # GENEric Package Manager
//!
//! Managing package managers to standardize package management.
//!
//! `args` is the main module for the GENEric Package Manager. It handles the command line arguments
//! and provides the structure for the package manager.
//!
//! ## Structures
//!
//! - `GeneArgs`: This structure holds the command line arguments for the GENEric Package Manager.
//!
//! ## Fields
//!
//! - `package_name`: The name of the package for the action.
//! - `backend`: The package manager to use. If not provided, it will be read from the gene-config file.
//! - `verbosity`: The level of verbosity. Default is 0.
//! - `config`: The path to the global GENE gene-config file. Default is "$XDG_CONFIG_HOME/gene/gene.toml".
//! - `local_config`: The local gene-config file to use.
//!
//! ## Usage
//!
//! This module is used by parsing the command line arguments and then performing the appropriate action
//! based on those arguments.
//!
//! ```
//! use gene::gene::GeneArgs;
//! let args = GeneArgs::parse();
//! ```
//!
//! The `GeneArgs` structure can then be used to perform the appropriate action.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct GeneArgs {
	/// Package name for action
	#[arg(value_name = "PACKAGE_NAME")]
	pub package_name: String,

	/// Package Manager to use (if not written to gene-config)
	#[arg(short, long)]
	pub backends: Option<Vec<String>>,

	/// Verbosity level
	#[arg(short, long, default_value = "0")]
	pub verbosity: i32,

	/// Path to global GENE gene-config file
	#[arg(short, long, default_value = "$XDG_CONFIG_HOME/gene/gene.toml")]
	pub config: String,

	/// Local gene-config file to use
	#[arg(short, long, default_value = "./gene.toml")]
	pub local_config: Option<String>,

	/// Query package
	#[arg(short, long)]
	pub query: bool,

	/// Remove package
	#[arg(short, long)]
	pub remove: bool,

	/// Quiet output
	#[arg(short = 'Q', long)]
	pub quiet: bool,

	/// Query packages
	#[arg(short, long)]
	pub search: bool,

	/// Upgrade package
	#[arg(short, long)]
	pub upgrade: bool,

	/// Raw Args to pass to backend
	#[arg(short = 'R', long)]
	// TODO: valiadate at least one backend is provided
	pub raw_args: Vec<String>,
}
