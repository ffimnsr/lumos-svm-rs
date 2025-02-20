use std::path::Path;
use std::sync::Arc;
use std::time::{
  Duration,
  Instant,
};

use clap::{
  Parser,
  Subcommand,
  crate_authors,
};
use hashbrown::HashMap;
use indicatif::{
  HumanDuration,
  ProgressBar,
};
use lumos_svm_lib::lumos_context::LumosContext;
use lumos_svm_lib::schema::ConfigRoot;
use lumos_svm_lib::traits::Pull;
use lumos_svm_lib::utils::validator;
use lumos_svm_lib::version::get_version_digits;
use once_cell::sync::Lazy;

use crate::analyze::Analyze;

/// Get the version digits
static VERSION: Lazy<String> = Lazy::new(get_version_digits);

/// Command line arguments
/// # Fields
/// * `config` - Config file to source
/// * `command` - Subcommand to run
#[derive(Debug, Parser)]
#[command(
  version = VERSION.as_str(),
  about,
  arg_required_else_help = true,
  author = crate_authors!("\n"),
  propagate_version = true,
)]
struct CliArgs {
  /// Config file to source
  #[arg(
    short,
    long,
    help = "Config file to source",
    env = "LUMOS_SVM_CONFIG",
    default_value = "lumos.toml"
  )]
  config: String,

  /// Subcommand to run
  #[command(subcommand)]
  command: Option<Command>,
}

/// Command line subcommands
/// # Variants
/// * `Clone` - Clone the programs, accounts, and data specified in the config
/// * `Run` - Run the a test solana validator
#[derive(Debug, Subcommand)]
enum Command {
  /// Clone the programs, accounts, and data specified in the config
  #[command(visible_aliases = ["c"], arg_required_else_help = false, about = "Clone the programs, accounts, and data specified in the config")]
  Clone {
    #[arg(short, long, help = "Clean the cache before cloning")]
    clean: bool,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
  },

  /// Run the a test solana validator
  #[command(visible_aliases = ["r"], arg_required_else_help = false, about = "Run the a test solana validator")]
  Run {
    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
  },

  /// Analyze given address
  #[command(visible_aliases = ["a"], arg_required_else_help = false, about = "Analyze given address")]
  Analyze(Analyze),
}

/// CliEntry is the main entry point for the CLI
pub struct CliEntry {
  args: CliArgs,
}

/// Implementation of CliEntry
impl CliEntry {
  /// Create a new CliEntry
  /// Returns an error if the config file is not found
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

  /// Run the command specified in the arguments
  /// Returns an error if the command is not found
  pub fn run(&self) -> anyhow::Result<()> {
    // Don't load the context here as it's not needed
    // It could simplify but it's not worth implementing
    // as not all command requires the context.
    match &self.args.command {
      Some(Command::Clone { clean, verbose }) => self.clone(*clean, *verbose),
      Some(Command::Run { verbose }) => self.run_validator(*verbose),
      Some(Command::Analyze(analyze)) => analyze.execute(&self.args.config),
      None => {
        anyhow::bail!("No subcommand provided. Use `--help` flag for more information.");
      },
    }
  }

  /// Clone the programs, accounts, and data specified in the config
  /// Returns an error if the config file is not found
  fn clone(&self, clean: bool, verbose: bool) -> anyhow::Result<()> {
    log::trace!("Cloning...");

    // Load the config
    let config = ConfigRoot::from_file(&self.args.config)?;
    let config = Arc::new(config);
    let rpc_endpoint: &str = &config.general.rpc_endpoint;
    let cache_dir = config.general.cache_dir.clone();

    // Create the context
    let context = LumosContext::new(config.clone(), rpc_endpoint, cache_dir, verbose);

    // Clean the cache if specified
    let cache_dir = context.cache_dir();
    let cache_dir = Path::new(&cache_dir);
    if cache_dir.exists() && clean {
      log::trace!("Cleaning cache...");
      std::fs::remove_dir_all(cache_dir)?;
    }

    // Don't thread this as it's using shared stdout and stdout
    // and it's not worth the complexity.
    let tick_interval = Duration::from_millis(80);

    // Clone accounts and programs
    self.clone_items(&context, &config.account, "account", tick_interval)?;
    self.clone_items(&context, &config.program, "program", tick_interval)?;

    Ok(())
  }

  fn clone_items<T: Pull>(
    &self,
    context: &LumosContext,
    items: &HashMap<String, T>,
    item_type: &str,
    tick_interval: Duration,
  ) -> anyhow::Result<()> {
    let started = Instant::now();

    let items_len = items.len();
    let pb = context.pb.add(ProgressBar::new(items_len as u64));
    pb.set_style(context.pb_style.clone());
    pb.enable_steady_tick(tick_interval);

    for (i, (name, item)) in items.iter().enumerate() {
      let message = format!("Cloning {}: {} ({})", item_type, item.address(), name);
      pb.set_prefix(format!("{}/{}", i + 1, items_len));
      pb.set_message(message);
      item.pull(context)?;
      pb.inc(1);
    }

    let message = format!(
      "Cloning {}s completed in {}.",
      item_type,
      HumanDuration(started.elapsed())
    );
    pb.finish_with_message(message);

    Ok(())
  }

  /// Run the a test solana validator
  fn run_validator(&self, verbose: bool) -> anyhow::Result<()> {
    log::trace!("Running validator...");

    // Start the timer
    let started = Instant::now();

    // Load the config
    let config = ConfigRoot::from_file(&self.args.config)?;
    let config = Arc::new(config);
    let rpc_endpoint: &str = &config.general.rpc_endpoint;
    let cache_dir = config.general.cache_dir.clone();

    // Create the context
    let context = LumosContext::new(config.clone(), rpc_endpoint, cache_dir, verbose);

    // Create the progress bar
    let tick_interval = Duration::from_millis(80);
    let pb = context.pb.add(ProgressBar::new(1));
    pb.set_style(context.pb_style.clone());
    pb.enable_steady_tick(tick_interval);
    pb.set_prefix("?/?");
    pb.set_message("Running validator...");

    // Run the validator
    validator(&context, true)?;

    // Finish the progress bar
    let message = format!(
      "Validator completed running in {}.",
      HumanDuration(started.elapsed())
    );
    pb.inc(1);
    pb.finish_with_message(message);
    Ok(())
  }
}
