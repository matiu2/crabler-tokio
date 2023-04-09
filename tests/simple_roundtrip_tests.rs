use crabler_tokio::*;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html("a[href]", print_handler)]
struct Scraper {
    visited_links: Arc<RwLock<Vec<String>>>,
    saw_links: Arc<RwLock<Vec<String>>>,
}

impl Scraper {
    async fn response_handler(&mut self, response: Response) -> Result<()> {
        self.visited_links.write().unwrap().push(response.url);
        Ok(())
    }

    async fn print_handler(&mut self, _response: Response, a: Element) -> Result<()> {
        if let Some(href) = a.attr("href") {
            self.saw_links.write().unwrap().push(href);
        } else {
            println!("Found link without an href");
        }

        Ok(())
    }
}

#[tokio::test]
async fn test_roundtrip() {
    let saw_links = Arc::new(RwLock::new(vec![]));
    let visited_links = Arc::new(RwLock::new(vec![]));

    let scraper = Scraper {
        visited_links: visited_links.clone(),
        saw_links: saw_links.clone(),
    };

    println!("before");
    scraper
        .run(Opts::default().with_urls(vec!["https://www.rust-lang.org/"]))
        .await
        .unwrap();
    println!("before");

    assert_eq!(visited_links.read().unwrap().len(), 1);
    println!("Found {} urls", saw_links.read().unwrap().len());
    assert!(saw_links.read().unwrap().len() > 10);
    assert_eq!(
        visited_links.read().unwrap().first().unwrap(),
        "https://www.rust-lang.org/"
    );
}
