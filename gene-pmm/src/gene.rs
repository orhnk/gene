use clap::Parser;

/// GENEric Package Manager
///
/// Managing package managers to standardize package management.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct GeneParams {
    /// Package name for action
    // #[arg(short, long)]
    name: String,

    /// Package Manager to use (if not written to gene-config)
    #[arg(short, long)]
    backend: Option<String>,

    /// Verbosity level
    #[arg(short, long, default_value = "0")]
    verbosity: i32,
    // -v -vv -vvv -> 1, 2, 3
    /// Path to global GENE gene-config file
    #[arg(short, long, default_value = "$XDG_CONFIG_HOME/gene/gene.toml")]
    config: Option<String>,

    /// Local gene-config file to use
    #[arg(short, long, default_value = "./gene.toml")]
    local_config: Option<String>,

    /// Quiet output
    #[arg(short, long)]
    quiet: bool,

    /// Query packages
    #[arg(short, long)]
    search: bool,
}