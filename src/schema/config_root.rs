use std::fs;
use std::path::Path;

use hashbrown::HashMap;
use serde::Deserialize;

use super::{
  AccountConfig,
  GeneralConfig,
  ProgramConfig,
};

/// ConfigRoot is a struct that holds the configuration of the lumos-svm
/// program.
#[derive(Debug, Deserialize, Default)]
pub struct ConfigRoot {
  /// General configuration
  pub general: GeneralConfig,
  /// List of svm accounts
  pub account: HashMap<String, AccountConfig>,
  /// List of svm programs
  pub program: HashMap<String, ProgramConfig>,
}

/// Implementation of ConfigRoot.
impl ConfigRoot {
  pub fn from_file(path: &str) -> anyhow::Result<Self> {
    let filepath = Path::new(path);
    if !filepath.exists() {
      anyhow::bail!("Config file not found: {}", path);
    }

    let contents = fs::read_to_string(filepath)?;
    let root: Self = toml::from_str(&contents)?;

    Ok(root)
  }
}
