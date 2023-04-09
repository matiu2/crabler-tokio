# Crabler-Tokio - Web crawler for Crabs

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[ci-badge]: https://github.com/matiu2/crabler-tokio/workflows/CI/badge.svg
[ci-url]: https://github.com/matiu2/crabler-tokio/actions
[crates-badge]: https://img.shields.io/crates/v/crabler-tokio.svg
[crates-url]: https://crates.io/crates/crabler-tokio
[docs-badge]: https://docs.rs/crabler-tokio/badge.svg
[docs-url]: https://docs.rs/crabler-tokio
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

Asynchronous web scraper engine written in Rust.

The original work and author can be found here: <https://github.com/Gonzih/crabler>

The main difference between crabler and crabler-tokio is that we've swapped out these dependencies:

 * async_std -> tokio
 * surf -> reqwest

The main motivation for this is to be closer to pure rust, and drop the requirments for libcurl and libssl.

The `cargo tree` output saves a few imports:

 * 488 dependencies for crabler
 * 350 dependencies for crabler-tokio

Features:
* fully based on `tokio`
* derive macro based API
* struct-based API
* stateful scraper (structs can hold state)
* ability to download files
* ability to schedule navigation jobs in an async manner
* uses the `reqwest` crate for HTTP requests

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
crabler-tokio = "0.1.0"

## Example

```rust,no_run
extern crate crabler;

use std::path::Path;

use crabler::*;
use reqwest::Url;

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html("a[href]", walk_handler)]
struct Scraper {}

impl Scraper {
    async fn response_handler(&self, response: Response) -> Result<()> {
        if response.url.ends_with(".png") && response.status == 200 {
            println!("Finished downloading {} -> {:?}", response.url, response.download_destination);
        }
        Ok(())
    }

    async fn walk_handler(&self, mut response: Response, a: Element) -> Result<()> {
        if let Some(href) = a.attr("href") {
            // Create absolute URL
            let url = Url::parse(&href)
                .unwrap_or_else(|_| Url::parse(&response.url).unwrap().join(&href).unwrap());

            // Attempt to download an image
            if href.ends_with(".png") {
                let image_name = url.path_segments().unwrap().last().unwrap();
                let p = Path::new("/tmp").join(image_name);
                let destination = p.to_string_lossy().to_string();

                if !p.exists() {
                    println!("Downloading {}", destination);
                    // Schedule crawler to download file to some destination
                    // downloading will happen in the background, await here is just to wait for job queue
                    response.download_file(url.to_string(), destination).await?;
                } else {
                    println!("Skipping existing file {}", destination);
                }
            } else {
              // Or schedule crawler to navigate to a given url
              response.navigate(url.to_string()).await?;
            };
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let scraper = Scraper {};

    // Run scraper starting from given url and using 20 worker threads
    scraper.run(Opts::new().with_urls(vec!["https://www.rust-lang.org/"]).with_threads(20)).await
}
```

## Sample project (for the original crabler crate)

[Gonzih/apod-nasa-scraper-rs](https://github.com/Gonzih/apod-nasa-scraper-rs/)
