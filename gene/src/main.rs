use std::collections::HashMap;
use clap::Parser;
use gene_utils::args::GeneArgs;
use gene_pmm::spm::pacman::*;
use gene_config::config::GeneTomlFormat;
use toml;
use toml::Value;


// NOTE: I wanted to use `anyhow` for error handling, but it doesn't work with limited lifetimes
// (dynamic dispatch makes it impossible to know what the lifetime of the variable is)

// TODO: improve overall error handling

fn main() {
	let config = r#"
	[system]
	backends = [
		"some",
	]
	"#;

	let config: GeneTomlFormat = toml::from_str(config).unwrap();

	println!("{:#?}", config);
}
