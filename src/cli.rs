use clap::Parser;

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
    output: String,
    url: String
}
