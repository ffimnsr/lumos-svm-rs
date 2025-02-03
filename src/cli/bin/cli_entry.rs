use std::path::Path;

use clap::{
  Parser,
  Subcommand,
  crate_authors,
};
use lumos_svm_lib::lumos_context::LumosContext;
use lumos_svm_lib::schema::ConfigRoot;

/// Command line arguments
/// # Fields
/// * `config` - Config file to source
/// * `command` - Subcommand to run
#[derive(Debug, Parser)]
#[command(
  about,
  arg_required_else_help = true,
  author = crate_authors!("\n"),
  propagate_version = true,
)]
struct CliArgs {
  #[arg(
    short,
    long,
    help = "Config file to source",
    env = "LUMOS_SVM_CONFIG",
    default_value = "lumos.toml"
  )]
  config: String,

  #[command(subcommand)]
  command: Option<Command>,
}

/// Command line subcommands
/// # Variants
/// * `Clone` - Clone the programs, accounts, and data specified in the config
/// * `Run` - Run the a test solana validator
#[derive(Debug, Subcommand)]
enum Command {
  #[command(visible_aliases = ["c"], arg_required_else_help = true, about = "Clone the programs, accounts, and data specified in the config")]
  Clone,

  #[command(visible_aliases = ["r"], arg_required_else_help = true, about = "Run the a test solana validator")]
  Run,
}

pub struct CliEntry {
  args: CliArgs,
}

impl CliEntry {
  pub fn new() -> anyhow::Result<Self> {
    let args = CliArgs::parse();
    log::trace!("Config: {}", args.config);

    assert!(!args.config.is_empty());

    let config = Path::new(&args.config);
    if !config.exists() {
      anyhow::bail!("Config file not found: {}", args.config);
    }

    Ok(Self { args })
  }

  pub fn run(&self) -> anyhow::Result<()> {
    match &self.args.command {
      Some(Command::Clone) => self.clone(),
      Some(Command::Run) => self.run_validator(),
      None => {
        anyhow::bail!("No command provided");
      },
    }
  }

  fn clone(&self) -> anyhow::Result<()> {
    log::info!("Cloning...");

    let config = ConfigRoot::from_file(&self.args.config)?;
    let rpc_endpoint: &str = &config.general.rpc_endpoint;
    let cache_dir = config.general.cache_dir;

    let context = LumosContext::new(rpc_endpoint, cache_dir, true);

    // Clone the accounts
    for (name, account) in config.account.iter() {
      log::info!("Cloning account: {}", name);
      account.pull(&context)?;
    }

    // Clone the programs
    for (name, program) in config.program.iter() {
      log::info!("Cloning program: {}", name);
      program.pull(&context)?;
    }

    Ok(())
  }

  fn run_validator(&self) -> anyhow::Result<()> {
    log::info!("Running validator...");
    Ok(())
  }
}
