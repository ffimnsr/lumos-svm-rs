use serde::Deserialize;

use crate::lumos_context::LumosContext;
use crate::traits::Pull;
use crate::utils::clone_program;

/// The program configuration definition.
#[derive(Debug, Deserialize)]
pub struct ProgramConfig {
  /// The public key address of the program.
  pub address: String,

  /// Set the authority of the program for upgradability.
  pub authority: Option<String>,

  /// Check if the program should be updated.
  pub update: Option<bool>,
}

impl Pull for ProgramConfig {
  /// Pull the program.
  fn pull(&self, context: &LumosContext) -> anyhow::Result<()> {
    let update = self.update.unwrap_or(false);
    clone_program(context, &self.address, update)
  }

  /// Get the address of the program.
  fn address(&self) -> &str {
    &self.address
  }
}
