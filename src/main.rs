extern crate reqwest;
extern crate rayon;

use rayon::prelude::*;

use std::io::{self, BufRead};
use std::env;

fn main() {
    let mut subs_vec = Vec::new();
    let concurrency: usize = 120;
    let text_vec: Vec<String> = env::args().skip(1).collect();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        subs_vec.push(line.unwrap());
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .build_global()
        .unwrap();

    if text_vec.is_empty() {
        eprintln!("Please provide title to check webpage for.\nUsage: cat domains.txt | checkr \"Webpage Title 1\" \"Webpage Title 2\"");
    } else {
        subs_vec.par_iter()
            .for_each(|sub|
                match client(sub, &text_vec) {
                    Ok(_) => (),
                    Err(_) => return
                }
            );
    }
}

fn client(sub: &str, vec: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get(sub).send()?;
    let body = res.text()?;

    for text in vec {
        if !body.contains(text) {
            println!("{}", sub);
        }
    }


    Ok(())
}
