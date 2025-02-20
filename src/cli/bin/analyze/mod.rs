use clap::Args;
use lumos_svm_lib::schema::ConfigRoot;
use lumos_svm_lib::solana_utils::get_token_details;
use tokio::runtime::Runtime;

#[derive(Debug, Args)]
pub struct Analyze {
  address: String,
}

impl Analyze {
  pub fn execute(&self, config_file: &str) -> anyhow::Result<()> {
    log::trace!("Analyzing address: {}", self.address);

    let config = ConfigRoot::from_file(config_file)?;
    let rpc_endpoint: &str = &config.general.rpc_endpoint;

    let rt = Runtime::new()?;
    let details = rt.block_on(async { get_token_details(rpc_endpoint, &self.address).await })?;

    println!("\nAnalysis:");
    println!("------------------------");
    println!("Owner: {}", details.owner);
    println!(
      "Mint Authority: {}",
      details.mint_authority.as_deref().unwrap_or("None")
    );
    println!(
      "Freeze Authority: {}",
      details.freeze_authority.as_deref().unwrap_or("None")
    );

    match (details.extensions, &details.metadata) {
      (true, Some(metadata)) => {
        println!(
          "Update Authority: {}",
          metadata.update_authority.as_deref().unwrap_or("None")
        );
        println!("Token Name: {}", metadata.name.as_deref().unwrap_or("None"));
        println!("Token Symbol: {}", metadata.symbol.as_deref().unwrap_or("None"));
        println!("Token URI: {}", metadata.uri.as_deref().unwrap_or("None"));
      },
      _ => println!("No metadata available"),
    }

    println!("Decimals: {}", details.decimals);
    println!("Supply: {}", details.supply);
    println!("Is Initialized: {}", details.is_initialized);
    println!("Extensions: {}", details.extensions);

    Ok(())
  }
}
