use crate::lumos_context::LumosContext;

/// Pull is a trait that defines the behavior of pulling data from a source.
pub trait Pull {
  fn pull(&self, context: &LumosContext) -> anyhow::Result<()>;
  fn address(&self) -> &str;
}
