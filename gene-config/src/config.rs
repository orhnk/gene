use std::path::{Path, PathBuf};
use crate::error::ConfigError;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneTomlFormat {
	/// [system.*] System configuration
	pub system: GeneSystemConfig, // required

	/// [plug.*] Plugin configuration
	pub plug: Option<toml::Value>,
	// `plug`s need to be flexible

	/// [config.*] GENE Configuration
	pub config: Option<GeneConfig>,
}

impl GeneTomlFormat {
	pub fn new() -> Self { Self::default() }

	pub fn compile(&mut self) -> Result<(), ConfigError> {
		self.prep_validate()?;
		todo!();
		Ok(())
	}

	fn prep_validate(&mut self) -> Result<(), ConfigError> {
		// TODO: discuss about whether to implement dual-typed backends?
		// ```rs
		// backend = [
		// 	"pacman",
		// 	"apt",
		// ]
		// ```
		// or
		// ```rs
		// backend = "pacman"
		// ```

		if self.config.is_none() {
			self.config = Some(GeneConfig::default());
		}

		let Some(backends) = self.system.backends.as_array()
			else { return Err(ConfigError::NoBackend); }; // if backends == null

		if backends.is_empty() { // if backends == []
			return Err(ConfigError::NoBackend);
		}
		// else if let Some(backends) = self.system.backends.as_str() {} else {
		// 		todo!("this feature is not implemented yet")
		// }

		Ok(())
	}
}

impl Default for GeneTomlFormat {
	fn default() -> Self {
		Self {
			system: GeneSystemConfig {
				backends: toml::Value::Array(vec!()),
				plugins: None,
			},
			config: Some(GeneConfig {
				config_file: Some(dirs::config_dir().expect("System Configuration Directory not found").join("gene/gene.toml")),
				..Default::default()
			}),
			..Default::default()
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneSystemConfig {
	pub backends: toml::Value,
	pub plugins: Option<toml::Value>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GeneConfig {
	verbosity: Option<i32>,
	log_file: Option<PathBuf>,
	config_file: Option<PathBuf>,
	// ... TODO: add more
}
