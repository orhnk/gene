//! # GENEric Package Manager
//!
//! Managing package managers to standardize package management.


use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct GeneArgs {
	/// Package name for action
	#[arg(value_name = "PACKAGE_NAME")]
	pub package_name: String,

	/// Package Manager to use (if not written to gene-config)
	#[arg(short, long)]
	pub backend: Option<String>,

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
	#[arg(short = 'R', long, requires = "backend")]
	// TODO: backend is unnecessary if There is only one backend in the config
	pub raw_args: Vec<String>,
}
