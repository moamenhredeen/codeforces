//! A. Watermelon
//!
//! Ref: https://codeforces.com/problemset/problem/4/A

use std::io::{self, BufRead};

fn main() {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    let weight: u32 = s.trim().parse().unwrap();
    let answer = if weight > 2 && weight % 2 == 0 {
        "YES"
    } else {
        "NO"
    };
    println!("{answer}");
}
