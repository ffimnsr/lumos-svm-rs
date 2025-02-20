use crate::lumos_context::LumosContext;

pub trait Pull {
  fn pull(&self, context: &LumosContext) -> anyhow::Result<()>;
  fn address(&self) -> &str;
}
