use clap::Parser;

use spider::cli::Cli;

fn main() {
    let cli = Cli::new();
    cli.start()
}
