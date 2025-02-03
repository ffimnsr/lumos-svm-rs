use serde::Deserialize;

/// The general configuration for the program.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
  /// The RPC endpoint to use.
  pub rpc_endpoint: String,

  /// The cache directory to use.
  pub cache_dir: Option<String>,
}
