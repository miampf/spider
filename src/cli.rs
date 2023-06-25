use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::thread;

use clap::Parser;
use colored::*;
use ratelimit::Ratelimiter;
use linkify::{LinkFinder, LinkKind};

#[derive(Parser, Debug, Default)]
#[clap(about="A simple program to crawl a website for other URLs.")]
pub struct Args {
    #[clap(short='m', long, help="Show mail addresses found.")]
    show_mail: bool,
    #[clap(short, long, help="Extend crawling to external URLs found.")]
    include_external_domains: bool,
    #[clap(short, long, default_value_t=5, help="How much requests per second should be performed.")]
    requests_per_second: u64,
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
    args: Args,
    link_finder: LinkFinder,
    discovered_pages: Vec<String>,
    discovered_emails: Vec<String>
}

impl Cli {
    /// Create a new cli and parse the command line arguments.
    pub fn new() -> Self {
        Self { 
            args: Args::parse(), 
            ..Default::default()
        }
    }

    /// Start the cli and the main loop.
    pub async fn start(&self) -> Result<(), crate::error::SpiderError>{
        if !self.args.no_banner {
            print!("{}", r#"
 ___  ____  ____  ____  ____  ____          |     |
/ __)(  _ \(_  _)(  _ \( ___)(  _ \         \     /
\__ \ )___/ _)(_  )(_) ))__)  )   /           UwU
(___/(__)  (____)(____/(____)(_)\_)          /   \
Made with <3 by miampf (github.com/miampf)  |     |
-----------------------------------------------------

"#.red().bold());
        }

        // construct the rate limited client
        let ratelimiter = Ratelimiter::builder(self.args.requests_per_second, Duration::from_secs(1))
            .max_tokens(self.args.requests_per_second)
            .build()?;

        /*
         * Tried to do this recursively but failed miserably at implementing
         * asynchronous recursion :(
         */
        self.spider(&ratelimiter).await?;    
        Ok(())
    }

    pub async fn spider(&self, ratelimiter: &Ratelimiter) -> Result<(), crate::error::SpiderError>{
        let to_scan = vec![self.args.url.clone()];
        let url_lock = Arc::new(RwLock::new(to_scan));

        let emails: Vec<String> = Vec::new();
        let email_lock = Arc::new(RwLock::new(emails));

        let client = reqwest::blocking::Client::new();

        if let Err(sleep) = ratelimiter.try_wait() {
            std::thread::sleep(sleep);
        }

        let ul = Arc::clone(&url_lock);
        let el = Arc::clone(&email_lock);

        let t = thread::spawn(move || {
            let to_scan = ul.read().unwrap();
            let url = to_scan.last().unwrap();
            let body = client.get(url.clone()).send().unwrap().text().unwrap();

            let mut finder = LinkFinder::new();
            finder.url_must_have_scheme(false);

            let links = finder.links(body.as_str());
            let mut to_scan = ul.write().unwrap();
            let mut emails = el.write().unwrap();
            for link in links {
                if link.kind() == &LinkKind::Url {
                    to_scan.push(link.as_str().to_string());
                } else if link.kind() == &LinkKind::Url {
                    emails.push(link.as_str().to_string());
                }
            }
        });

        t.join().unwrap();
        println!("{:?}\n{:?}", url_lock, email_lock);

        Ok(())
    }
}
