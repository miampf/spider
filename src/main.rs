use clap::Parser;

use spider::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), crate::error::SpiderError> {
    let cli = Cli::new();
    cli.start().await?

    Ok(())
}
