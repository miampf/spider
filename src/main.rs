use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use spider::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), spider::error::SpiderError> {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set up logging subscriber");

    let cli = Cli::new();
    cli.start().await?;

    Ok(())
}
