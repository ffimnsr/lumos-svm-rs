use cli_entry::CliEntry;

mod cli_entry;

/// Main entry point
fn main() -> anyhow::Result<()> {
    let cli = CliEntry::new()?;
    cli.run()
}
