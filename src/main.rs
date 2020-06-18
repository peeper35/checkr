use std::io::{self, BufRead};

fn main() {
    let mut subs_vec = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        subs_vec.push(line.unwrap());
    }

}
