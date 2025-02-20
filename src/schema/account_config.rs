use serde::Deserialize;

use crate::lumos_context::LumosContext;
use crate::traits::Pull;
use crate::utils::clone_account;

/// The account configuration definition.
#[derive(Debug, Deserialize)]
pub struct AccountConfig {
  /// The public key address of the account.
  pub address: String,

  /// Check if the account should be updated.
  pub update: Option<bool>,
}

/// An implementation of the account configuration.
impl Pull for AccountConfig {
  /// Pulls the account configuration.
  fn pull(&self, context: &LumosContext) -> anyhow::Result<()> {
    let update = self.update.unwrap_or(false);
    clone_account(context, &self.address, update)
  }

  /// Get the address of the account.
  fn address(&self) -> &str {
    &self.address
  }
}
