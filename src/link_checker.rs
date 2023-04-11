use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;


static NAME_WIDTH: usize = 10;
static ID_WIDTH: usize = 3;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}


struct Checker {
    id: u8,
    valid_urls_tx: mpsc::Sender<Url>,
    new_urls_rx: Arc<Mutex<mpsc::Receiver<Url>>>,
}

impl Checker {
    fn run(&self) {
        println!("{: ^NAME_WIDTH$} | {: ^ID_WIDTH$} | started", "Checker", self.id);
        while let Ok(url) = self.new_urls_rx.lock().unwrap().recv() {
            println!("{: ^NAME_WIDTH$} | {: ^ID_WIDTH$} | fetching {url}", "Checker", self.id);
            let response = get(url).unwrap();
            let base_url = response.url().to_owned();
            let document = response.text().unwrap();
            let html = Html::parse_document(&document);
            let selector = Selector::parse("a").unwrap();
            let valid_urls = html
                .select(&selector)
                .filter_map(|element| element.value().attr("href"))
                .map(|href| base_url.join(href))
                .filter_map(Result::ok)
                .for_each(|url|
                    self.valid_urls_tx.send(url).unwrap()
                );
        }
        println!("{: ^NAME_WIDTH$} | {: ^ID_WIDTH$} | finished", "Checker", self.id);
    }
}


pub fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();

    let (valid_urls_tx, valid_urls_rx) = mpsc::channel::<Url>();
    let (new_urls_tx, new_urls_rx) = mpsc::channel::<Url>();
    let new_urls_rx = Arc::new(Mutex::new(new_urls_rx));

    let chekers_handlers = (0..5)
        .map(|id| {
            let checker = Checker {
                id,
                valid_urls_tx: valid_urls_tx.clone(),
                new_urls_rx: new_urls_rx.clone(),
            };
            std::thread::spawn(move || checker.run())
        })
        .collect::<Vec<_>>();

    new_urls_tx.send(start_url).unwrap();
    let mut limit = 10;
    while let Ok(url) = valid_urls_rx.recv() {
        println!("{: ^NAME_WIDTH$} | {limit: ^ID_WIDTH$} | got url: {}", "main", url);
        if limit == 0 {
            break;
        } else {
            limit -= 1;
        }
        new_urls_tx.send(url).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    drop(new_urls_tx);

    for handler in chekers_handlers {
        handler.join().unwrap();
    }
}