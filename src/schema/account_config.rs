use serde::Deserialize;

use crate::lumos_context::LumosContext;
use crate::utils::clone_account;

/// The account configuration definition.
#[derive(Debug, Deserialize)]
pub struct AccountConfig {
  /// The public key address of the account.
  pub address: String,
}

impl AccountConfig {
  pub fn pull(&self, context: &LumosContext) -> anyhow::Result<()> {
    println!("Pulling account: {}", self.address);
    clone_account(context, &self.address)
  }
}
