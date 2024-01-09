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
    /// Package Manager Backend to use
    #[arg()]
    pub package_manager: String,

    /// Package Manager Args
    #[arg()]
    pub args: Vec<String>,

    /// Unchecked Raw Arguments
    #[arg()]
    pub raw_args: Vec<String>,
}
