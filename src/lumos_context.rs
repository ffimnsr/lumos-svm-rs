use indicatif::MultiProgress;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::file::ToUtf8 as _;

/// LumosContext is a struct that holds the configuration of the lumos-svm program.
pub struct LumosContext {
  rpc_endpoint: Rc<RefCell<String>>,
  cache_dir: Option<String>,
  pub pb: Arc<MultiProgress>,
  pub verbose: bool,
}

/// Implementation of LumosContext.
impl LumosContext {
  pub fn new(rpc_endpoint: &str, cache_dir: Option<String>, verbose: bool) -> Self {
    Self {
      rpc_endpoint: Rc::new(RefCell::new(rpc_endpoint.into())),
      cache_dir,
      pb: Arc::new(MultiProgress::new()),
      verbose,
    }
  }

  pub fn set_rpc_endpoint(&self, rpc_endpoint: String) {
    *self.rpc_endpoint.borrow_mut() = rpc_endpoint;
  }

  pub fn rpc_endpoint(&self) -> String {
    self.rpc_endpoint.borrow().clone()
  }

  pub fn cache_dir(&self) -> String {
    self.cache_dir.clone().unwrap_or(".lumos-cache".into())
  }

  pub fn program_cache_dir(&self) -> anyhow::Result<String> {
    self.cache_dir_join("programs")
  }

  pub fn account_cache_dir(&self) -> anyhow::Result<String> {
    self.cache_dir_join("accounts")
  }

  fn cache_dir_join(&self, p: &str) -> anyhow::Result<String> {
    let cache_dir = self.cache_dir();
    let path = Path::new(&cache_dir);
    let path = path.join(p);
    let path = path.to_utf8()?;
    Ok(path.to_string())
  }
}
