use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::thread;

use clap::Parser;
use colored::*;
use ratelimit::Ratelimiter;
use linkify::{LinkFinder, LinkKind};
use tracing::{info, error, Level};
use url::Url;

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
}

impl Cli {
    /// Create a new cli and parse the command line arguments.
    pub fn new() -> Self {
        Self { 
            args: Args::parse(), 
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

        let mut threads = Vec::new();
        let mut thread_number = 0; // Works like an id for the thread. Useful for debugging and
                                   // similair
        loop {
            if let Err(sleep) = ratelimiter.try_wait() {
                thread_number = 0;
                std::thread::sleep(sleep);
            }

            let ul = Arc::clone(&url_lock);
            let el = Arc::clone(&email_lock);
            let orig_url = self.args.url.clone();
            let include_external = self.args.include_external_domains;

            // This is the thread that will actually make the requests
            threads.push(thread::spawn(move || {
                let _s = tracing::span!(Level::INFO, "http_request_thread", thread_number).entered();

                // aquire a read lock for to_scan
                let to_scan = ul.read();
                if to_scan.is_err() {
                    error!("Failed to get read lock for to_scan: {}", to_scan.unwrap_err());
                    return;
                }
                let to_scan = to_scan.unwrap();
                
                let tc_clone = to_scan.clone();
                let url = tc_clone.last();
                if url.is_none() {
                    error!("No URLs left to scan, exiting thread");
                    return;
                }
                let url = url.unwrap();

                let res = reqwest::blocking::get(url.clone());
                if res.is_err() {
                    error!("Request failed: {}", res.unwrap_err());
                    return;
                }
                let res = res.unwrap();

                let body = res.text();
                if body.is_err() {
                    error!("Failed to get body from response: {}", body.unwrap_err());
                    return;
                }
                let body = body.unwrap();

                let finder = LinkFinder::new();
                
                // release the read lock to prevent a deadlock
                drop(to_scan);

                let links = finder.links(body.as_str());

                // aquire a write lock for to_scan
                let to_scan = ul.write();
                if to_scan.is_err() {
                    error!("Failed to get write lock for to_scan: {}", to_scan.unwrap_err());
                    return;
                }
                let mut to_scan = to_scan.unwrap();

                // aquire a write lock for emails
                let emails = el.write();
                if emails.is_err() {
                    error!("Failed to get write lock for emails: {}", emails.unwrap_err());
                    return;
                }
                let mut emails = emails.unwrap();

                // extract the links and emails
                for link in links {
                    if link.kind() == &LinkKind::Url {
                        info!("Found link: {}", link.as_str());
                        
                        if Url::parse(link.as_str()).unwrap().host_str() == Some(orig_url.as_str()) || include_external {
                        to_scan.push(link.as_str().to_string());
                        }
                    } else if link.kind() == &LinkKind::Url {
                        info!("Found email: {}", link.as_str());
                        emails.push(link.as_str().to_string());
                    }
                }

                // remove all elements from to_scan that were this url
                to_scan.retain(|x| x != url);
            }));

            for i in 0..threads.len() {
                let t = threads.get(i);
                if t.is_none() {
                    break;
                }
                if t.unwrap().is_finished() {
                   threads.remove(i);
               } 
            }

            if threads.is_empty() && url_lock.read().unwrap().is_empty(){
                break;
            }

            thread_number += 1;
        }

        Ok(())
    }
}
