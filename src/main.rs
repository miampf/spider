use spider::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), spider::error::SpiderError> {
    let cli = Cli::new();
    cli.start().await?;

    Ok(())
}
