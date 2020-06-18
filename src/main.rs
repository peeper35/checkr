extern crate reqwest;

use std::io::{self, BufRead};
use std::error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut subs_vec = Vec::new();
    let text_vec: Vec<String> = env::args().skip(1).collect();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        subs_vec.push(line.unwrap());
    }

    if text_vec.is_empty() {
        eprintln!("Please provide title to check webpage for.\nUsage: cat domains.txt | checkr \"Webpage Title 1\" \"Webpage Title 2\"")
    } else {
        for sub in subs_vec {
            client(&sub, &text_vec).await?;
        }
    }

    Ok(())

}

async fn client(sub: &str, vec: &Vec<String>) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get(sub).send().await?;
    let body = res.text().await?;

    for text in vec {
        if !body.contains(text) {
            println!("{}", sub);
        }
    }

    Ok(())
}
