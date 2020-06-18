extern crate reqwest;

use reqwest::Client;

use std::io::{self, BufRead};
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut subs_vec = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        subs_vec.push(line.unwrap());
    }

    for sub in subs_vec {
        println!("{}", sub);
        client(&sub).await?;
    }

    Ok(())

}

async fn client(sub: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get(sub).send().await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}
