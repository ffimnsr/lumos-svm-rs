use anyhow::Context;
use base64::prelude::*;
use serde::{
  Deserialize,
  Serialize,
};
use std::fs;
use std::io::{
  BufRead as _,
  BufReader,
};
use std::net::TcpListener;
use std::path::Path;
use std::process::{
  Command,
  Stdio,
};
use which::which;

use crate::file::ToUtf8;
use crate::lumos_context::LumosContext;
use crate::traits::Pull as _;

#[derive(Debug, Deserialize, Serialize)]
struct AccountDataRoot {
  pub pubkey: String,
  pub account: AccountData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountData {
  pub lamports: i64,
  pub data: Vec<String>,
  pub owner: String,
  pub executable: bool,
  pub rent_epoch: i64,
  pub space: i64,
}

/// Clone an account.
pub fn clone_account(context: &LumosContext, address: &str, update: bool, mint: bool) -> anyhow::Result<()> {
  let solana_cmd = which("solana").with_context(|| "Failed to find solana command")?;

  if !is_valid_solana_address(address) {
    anyhow::bail!("Invalid Solana address: {}", address);
  }

  let stdout = get_tty_output!(context.verbose);
  let stderr = Stdio::piped();

  let cache_dir: &str = &context.account_cache_dir()?;
  let cache_dir = Path::new(cache_dir);

  // Create the cache directory if it doesn't exist.
  if !cache_dir.exists() {
    fs::create_dir_all(cache_dir)?;
  }

  let out_filename: &str = &format!("{address}.json");
  let out_file = cache_dir.join(out_filename);

  // If the file already exists and we're not updating, then return early.
  if out_file.exists() && !update {
    return Ok(());
  }

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

  if context.verbose {
    handle_tty_output!(cmd.stdout, context);
  }

  handle_tty_output!(cmd.stderr, context);

  let status = cmd.wait()?;
  if !status.success() {
    anyhow::bail!("Failed to clone account: {address}");
  }

  // If mint is true, then modify the mint account.
  if mint {
    let pb = context.pb.clone();

    // Open the account file, modify it to be a mint account, and write it back
    let account_data = fs::read_to_string(out_file)?;
    let mut account_json: AccountDataRoot = serde_json::from_str(&account_data)
      .with_context(|| format!("Failed to parse account JSON for {}", address))?;

    // Get the base64 data from the account
    if account_json.account.data.is_empty() {
      anyhow::bail!("Account data is empty for mint: {}", address);
    }

    // Decode the base64 data
    let data_str = &account_json.account.data[0];
    let mut data = BASE64_STANDARD
      .decode(data_str)
      .with_context(|| format!("Failed to decode base64 data for mint: {}", address))?;

    // Log the original authority
    if context.verbose && data.len() >= 36 {
      let msg = format!("Original authority: {}", bs58::encode(&data[4..36]).into_string());
      let _ = pb.println(msg);
    }

    // Replace the mint authority (bytes 4-36) with a new authority
    // For this example, we'll use the context's wallet address or a default
    let authority: &str = &context
      .mint_authority()
      .with_context(|| "Mint authority is not set in the `general` configuration")?;

    if !is_valid_solana_address(authority) {
      anyhow::bail!("Invalid mint authority address: {}", authority);
    }

    let authority_bytes = bs58::decode(authority)
      .into_vec()
      .with_context(|| format!("Failed to decode authority address: {}", authority))?;

    // Ensure we have enough data and replace the authority bytes
    if data.len() >= 36 && authority_bytes.len() == 32 {
      data[4..36].copy_from_slice(&authority_bytes);
    } else {
      anyhow::bail!("Invalid data length or authority bytes for mint: {}", address);
    }

    // Log the new authority
    if context.verbose {
      let msg = format!("New authority: {}", bs58::encode(&data[4..36]).into_string());
      let _ = pb.println(msg);
    }

    // Update the account data with the modified bytes
    account_json.account.data[0] = BASE64_STANDARD.encode(&data);

    // Write the modified JSON back to the file
    fs::write(out_file, serde_json::to_string_pretty(&account_json)?)?;
  }

  Ok(())
}

/// Clone a program.
pub fn clone_program(context: &LumosContext, address: &str, update: bool) -> anyhow::Result<()> {
  let solana_cmd = which("solana").with_context(|| "Failed to find solana command")?;

  if !is_valid_solana_address(address) {
    anyhow::bail!("Invalid Solana address: {}", address);
  }

  let stdout = get_tty_output!(context.verbose);
  let stderr = Stdio::piped();

  let cache_dir: &str = &context.program_cache_dir()?;
  let cache_dir = Path::new(cache_dir);

  // Create the cache directory if it doesn't exist.
  if !cache_dir.exists() {
    fs::create_dir_all(cache_dir)?;
  }

  let out_filename: &str = &format!("{address}.so");
  let out_file = cache_dir.join(out_filename);

  // If the file already exists and we're not updating, then return early.
  if out_file.exists() && !update {
    return Ok(());
  }

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

  if context.verbose {
    handle_tty_output!(cmd.stdout, context);
  }

  handle_tty_output!(cmd.stderr, context);

  let status = cmd.wait()?;
  if !status.success() {
    anyhow::bail!("Failed to clone program: {address}");
  }

  Ok(())
}

/// Start a validator.
pub fn validator(context: &LumosContext, reset: bool) -> anyhow::Result<()> {
  let solana_test_validator_cmd =
    which("solana-test-validator").with_context(|| "Failed to find solana-test-validator command")?;

  let stdout = get_tty_output!(context.verbose);
  let stderr = Stdio::piped();

  let rpc_endpoint: &str = &context.rpc_endpoint();
  let ledger_dir: &str = &context
    .config
    .general
    .ledger_dir
    .clone()
    .unwrap_or(".lumos-ledger".into());

  let mut cmd = Command::new(solana_test_validator_cmd);
  cmd
    .stdout(stdout)
    .stderr(stderr)
    .arg("--url")
    .arg(rpc_endpoint)
    .arg("--ledger")
    .arg(ledger_dir);

  // Check if the rpc port is available, if not, then use a different port.
  if !is_validator_port_available(8899) {
    cmd.arg("--rpc-port").arg("8900");
  }

  // Check if faucet port is available, if not, then use a different port.
  if !is_validator_port_available(9900) {
    cmd.arg("--faucet-port").arg("9901");
  }

  // Process the accounts
  let account_cache_dir: &str = &context.account_cache_dir()?;
  let account_cache_dir = Path::new(account_cache_dir);
  let account_cache_dir: &str = account_cache_dir.to_utf8()?;

  // Pull the accounts, if any.
  for (_, account) in context.config.account.iter() {
    account.pull(context)?;
  }

  // Add the accounts to the validator.
  cmd.arg("--account-dir").arg(account_cache_dir);

  // Process the programs and add them to the validator.
  for (_, program) in context.config.program.iter() {
    let address: &str = &program.address;

    // Pull the program, if any.
    program.pull(context)?;

    let cache_dir: &str = &context.program_cache_dir()?;
    let cache_dir = Path::new(cache_dir);
    let out_filename: &str = &format!("{address}.so");
    let out_file = cache_dir.join(out_filename);

    // If it doesn't exist, then skip.
    if !out_file.exists() {
      continue;
    }

    let out_file: &str = out_file.to_utf8()?;

    // If the program has an authority, then use the upgradeable-program flag.
    if let Some(authority) = &program.authority {
      cmd
        .arg("--upgradeable-program")
        .arg(address)
        .arg(out_file)
        .arg(authority);
    } else {
      cmd.arg("--bpf-program").arg(address).arg(out_file);
    }
  }

  if reset {
    cmd.arg("--reset");
  }

  let mut cmd = cmd.spawn()?;

  if context.verbose {
    handle_tty_output!(cmd.stdout, context);
  }
  handle_tty_output!(cmd.stderr, context);

  let status = cmd.wait()?;
  if !status.success() {
    anyhow::bail!("Failed to start validator");
  }

  Ok(())
}

/// Check if the validator port is available.
/// Returns true if the port is available, false otherwise.
/// # Arguments
/// * `port` - The port to check.
fn is_validator_port_available(port: u16) -> bool {
  TcpListener::bind(("0.0.0.0", port)).is_ok()
}

/// Checks if a string is a valid Solana address.
/// A valid Solana address is a Base58-encoded string that decodes to exactly 32 bytes.
fn is_valid_solana_address(address: &str) -> bool {
  if address.is_empty() {
    return false;
  }

  // Try to decode the Base58 string
  match bs58::decode(address).into_vec() {
    Ok(bytes) => bytes.len() == 32, // Solana addresses should be 32 bytes
    Err(_) => false,
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use crate::schema::ConfigRoot;

  use super::*;
  use assert_fs::TempDir;

  #[test]
  fn it_should_clone_account_and_output_json_file() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let address = "AKEWE7Bgh87GPp171b4cJPSSZfmZwQ3KaqYqXoKLNAEE";
    let cache_dir = temp_dir.path();
    let config = Arc::new(ConfigRoot::default());
    let rpc_endpoint = "https://eclipse.lgns.net/";

    let context = LumosContext::new(
      config,
      rpc_endpoint,
      Some(cache_dir.to_str().context("Unable to unwrap cache dir")?.into()),
      false,
    );

    clone_account(&context, address, false, false)?;

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
    let config = Arc::new(ConfigRoot::default());
    let rpc_endpoint = "https://eclipse.lgns.net/";

    let context = LumosContext::new(
      config,
      rpc_endpoint,
      Some(cache_dir.to_str().context("Unable to unwrap cache dir")?.into()),
      false,
    );

    clone_program(&context, address, false)?;

    let out_filename: &str = &format!("{address}.so");
    let out_file = cache_dir.join("programs").join(out_filename);
    assert!(out_file.exists());
    Ok(())
  }
}
