use serde::Deserialize;

use crate::{lumos_context::LumosContext, utils::clone_program};

/// The program configuration definition.
#[derive(Debug, Deserialize)]
pub struct ProgramConfig {
  /// The public key address of the program.
  pub address: String,
}

impl ProgramConfig {
  pub fn pull(&self, context: &LumosContext) -> anyhow::Result<()> {
    log::trace!("Pulling program: {}", self.address);
    clone_program(context, &self.address)
  }
}
