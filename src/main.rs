use clap::Parser;

use spider::cli::Args;

fn main() {
   let args = Args::parse();
   println!("{:?}", args)
}
