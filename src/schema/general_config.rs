use serde::Deserialize;

/// The general configuration for the program.
#[derive(Debug, Deserialize, Default)]
pub struct GeneralConfig {
  /// The RPC endpoint to use.
  pub rpc_endpoint: String,

  /// The cache directory to use.
  pub cache_dir: Option<String>,

  /// Validator ledger directory.
  pub ledger_dir: Option<String>,

  /// The mint authority.
  /// This is the public key of the mint authority that will
  /// be use to override the tokens mint authority.
  pub mint_authority: Option<String>,
}
