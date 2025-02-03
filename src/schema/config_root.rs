use std::fs;
use std::path::Path;

use hashbrown::HashMap;
use serde::Deserialize;

use super::{
  AccountConfig,
  GeneralConfig,
  ProgramConfig,
};

#[derive(Debug, Deserialize)]
pub struct ConfigRoot {
  pub general: GeneralConfig,
  pub account: HashMap<String, AccountConfig>,
  pub program: HashMap<String, ProgramConfig>,
}

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
