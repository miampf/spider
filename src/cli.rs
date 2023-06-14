use clap::Parser;
use colored::*;

#[derive(Parser, Debug, Default)]
#[clap(about="A simple program to crawl a website for other URLs.")]
pub struct Args {
    #[clap(short, long, default_value_t=5, help="The recursion limit.")]
    recursion: u8,
    #[clap(short='m', long, help="Show mail addresses found.")]
    show_mail: bool,
    #[clap(short, long, help="Extend crawling to external URLs found.")]
    include_external_domains: bool,
    #[clap(short, long, help="Save all files found. This will create a new directory with the website name.")]
    save_files: bool,
    #[clap(short, long, help="A filename to which the valid URLs are written. This also saves email addresses if showing mail addresses is enabled.")]
    output: Option<String>,
    #[clap(short, long, help="Don't print the banner at the beginning.")]
    no_banner: bool,
    url: String
}

#[derive(Debug, Default)]
pub struct Cli {
    args: Args
}

impl Cli {
    /// Create a new cli and parse the command line arguments.
    pub fn new() -> Self {
        Self { args: Args::parse() }
    }

    /// Start the cli and the main loop.
    pub fn start(&self) {
        if !self.args.no_banner {
            print!("{}", r#"
 ___  ____  ____  ____  ____  ____          |     |
/ __)(  _ \(_  _)(  _ \( ___)(  _ \         \     /
\__ \ )___/ _)(_  )(_) ))__)  )   /           UwU
(___/(__)  (____)(____/(____)(_)\_)          /   \
Made with <3 by miampf (github.com/miampf)  |     |
-----------------------------------------------------
 "#.red().bold()) 
        }
    }
}
