use std::ffi::OsStr;
use std::path::Path;

/// This has been adapted from cross-rs file.rs source
/// https://github.com/cross-rs/cross/blob/4090beca3cfffa44371a5bba524de3a578aa46c3/src/file.rs#L12
pub trait ToUtf8 {
  /// Convert the type to a UTF-8 string
  fn to_utf8(&self) -> anyhow::Result<&str>;
}

/// Implement the trait for OsStr
impl ToUtf8 for OsStr {
  /// Convert the OsStr to a UTF-8 string
  fn to_utf8(&self) -> anyhow::Result<&str> {
    self
      .to_str()
      .ok_or_else(|| anyhow::anyhow!("Unable to convert `{self:?}` to UTF-8 string"))
  }
}

/// Implement the trait for Path
impl ToUtf8 for Path {
  /// Convert the Path to a UTF-8 string
  fn to_utf8(&self) -> anyhow::Result<&str> {
    self.as_os_str().to_utf8()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_convert_str_to_utf8() -> anyhow::Result<()> {
    let os_str = OsStr::new("hello");
    assert_eq!(os_str.to_utf8()?, "hello");
    Ok(())
  }

  #[test]
  fn it_should_convert_path_to_utf8() -> anyhow::Result<()> {
    let path = Path::new("hello");
    assert_eq!(path.to_utf8()?, "hello");
    Ok(())
  }
}
