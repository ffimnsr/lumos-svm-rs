use anyhow::Context;
use std::fs;
use std::io::{
  BufRead as _,
  BufReader,
};
use std::path::Path;
use std::process::Command;
use which::which;

use crate::file::ToUtf8;
use crate::lumos_context::LumosContext;

pub fn clone_account(context: &LumosContext, address: &str) -> anyhow::Result<()> {
  let solana_cmd = which("solana").with_context(|| "Failed to find solana command")?;

  let stdout = get_tty_output!(context.verbose);
  let stderr = get_tty_output!(context.verbose);

  let cache_dir: &str = &context.account_cache_dir()?;
  let cache_dir = Path::new(cache_dir);
  if !cache_dir.exists() {
    fs::create_dir_all(cache_dir)?;
  }

  let out_filename: &str = &format!("{address}.json");
  let out_file = cache_dir.join(out_filename);
  let out_file: &str = out_file.to_utf8()?;

  let rpc_endpoint: &str = &context.rpc_endpoint();
  let mut cmd = Command::new(solana_cmd);
  cmd
    .arg("account")
    .arg(address)
    .arg("--output")
    .arg("json")
    .arg("--url")
    .arg(rpc_endpoint)
    .arg("--output-file")
    .arg(out_file)
    .stdout(stdout)
    .stderr(stderr);

  let mut cmd = cmd.spawn()?;

  // if context.verbose {
  //   handle_tty_output!(cmd.stdout, context);
  //   handle_tty_output!(cmd.stderr, context);
  // }

  let status = cmd.wait()?;
  if !status.success() {
    anyhow::bail!("Failed to clone account: {address}");
  }

  Ok(())
}

pub fn clone_program(context: &LumosContext, address: &str) -> anyhow::Result<()> {
  let solana_cmd = which("solana").with_context(|| "Failed to find solana command")?;

  let stdout = get_tty_output!(context.verbose);
  let stderr = get_tty_output!(context.verbose);

  let cache_dir: &str = &context.program_cache_dir()?;
  let cache_dir = Path::new(cache_dir);
  if !cache_dir.exists() {
    fs::create_dir_all(cache_dir)?;
  }

  let out_filename: &str = &format!("{address}.so");
  let out_file = cache_dir.join(out_filename);
  let out_file: &str = out_file.to_utf8()?;

  let rpc_endpoint: &str = &context.rpc_endpoint();
  let mut cmd = Command::new(solana_cmd);
  cmd
    .arg("program")
    .arg("dump")
    .arg(address)
    .arg(out_file)
    .arg("--url")
    .arg(rpc_endpoint)
    .stdout(stdout)
    .stderr(stderr);

  let mut cmd = cmd.spawn()?;

  // if context.verbose {
  //   handle_tty_output!(cmd.stdout, context);
  //   handle_tty_output!(cmd.stderr, context);
  // }

  let status = cmd.wait()?;
  if !status.success() {
    anyhow::bail!("Failed to clone program: {address}");
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use assert_fs::TempDir;

  #[test]
  fn it_should_clone_account_and_output_json_file() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let address = "AKEWE7Bgh87GPp171b4cJPSSZfmZwQ3KaqYqXoKLNAEE";
    let cache_dir = temp_dir.path();
    let rpc_endpoint = "https://eclipse.lgns.net/";

    let context = LumosContext::new(
      rpc_endpoint,
      Some(cache_dir.to_str().context("Unable to unwrap cache dir")?.into()),
      false,
    );

    clone_account(&context, address)?;

    let out_filename: &str = &format!("{address}.json");
    let out_file = cache_dir.join("accounts").join(out_filename);
    assert!(out_file.exists());
    Ok(())
  }

  #[test]
  fn it_should_clone_program_and_output_so_file() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let address = "br1xwubggTiEZ6b7iNZUwfA3psygFfaXGfZ1heaN9AW";
    let cache_dir = temp_dir.path();
    let rpc_endpoint = "https://eclipse.lgns.net/";

    let context = LumosContext::new(
      rpc_endpoint,
      Some(cache_dir.to_str().context("Unable to unwrap cache dir")?.into()),
      false,
    );

    clone_program(&context, address)?;

    let out_filename: &str = &format!("{address}.so");
    let out_file = cache_dir.join("programs").join(out_filename);
    assert!(out_file.exists());
    Ok(())
  }
}
