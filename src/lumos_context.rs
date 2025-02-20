use indicatif::{
  MultiProgress,
  ProgressStyle,
};
use std::path::Path;
use std::sync::{
  Arc,
  RwLock,
};

use crate::file::ToUtf8 as _;
use crate::schema::ConfigRoot;

/// LumosContext is a struct that holds the configuration of the lumos-svm program.
pub struct LumosContext {
  rpc_endpoint: Arc<RwLock<String>>,
  cache_dir: Option<String>,
  pub config: Arc<ConfigRoot>,
  pub pb: Arc<MultiProgress>,
  pub pb_style: ProgressStyle,
  pub verbose: bool,
}

/// Implementation of LumosContext.
impl LumosContext {
  /// Create a new LumosContext.
  pub fn new(config: Arc<ConfigRoot>, rpc_endpoint: &str, cache_dir: Option<String>, verbose: bool) -> Self {
    Self {
      config: config.clone(),
      rpc_endpoint: Arc::new(RwLock::new(rpc_endpoint.into())),
      cache_dir,
      pb: Arc::new(MultiProgress::new()),
      pb_style: ProgressStyle::with_template("{spinner:.green} [{prefix:.bold.dim}] {wide_msg:.cyan/blue} ")
        .expect("Failed to create progress style")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏⦿"),
      verbose,
    }
  }

  /// Set the RPC endpoint.
  pub fn set_rpc_endpoint(&self, rpc_endpoint: String) -> anyhow::Result<()> {
    self
      .rpc_endpoint
      .write()
      .map(|mut endpoint| *endpoint = rpc_endpoint)
      .map_err(|_| anyhow::anyhow!("Failed to set rpc endpoint"))
  }

  /// Get the RPC endpoint.
  pub fn rpc_endpoint(&self) -> String {
    self
      .rpc_endpoint
      .read()
      .map(|endpoint| endpoint.clone())
      .unwrap_or_else(|_| "http://localhost:8899".into())
  }

  /// Get the cache directory.
  pub fn cache_dir(&self) -> String {
    self.cache_dir.clone().unwrap_or(".lumos-cache".into())
  }

  /// Get the program cache directory.
  pub fn program_cache_dir(&self) -> anyhow::Result<String> {
    self.cache_dir_join("programs")
  }

  /// Get the account cache directory.
  pub fn account_cache_dir(&self) -> anyhow::Result<String> {
    self.cache_dir_join("accounts")
  }

  /// Join the cache directory with a path.
  fn cache_dir_join(&self, p: &str) -> anyhow::Result<String> {
    let cache_dir = self.cache_dir();
    let path = Path::new(&cache_dir);
    let path = path.join(p);
    let path = path.to_utf8()?;
    Ok(path.to_string())
  }
}
