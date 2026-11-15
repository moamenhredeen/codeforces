//! 50A - Domino Piling
//!
//! Ref: https://codeforces.com/problemset/problem/50/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(mut input: R, mut out: W) {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let board: u32 = s
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .product();
    let dominos = board / 2;
    writeln!(out, "{dominos}").unwrap();
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    solve(stdin.lock(), BufWriter::new(stdout.lock()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> String {
        let mut out = Vec::new();
        solve(input.as_bytes(), &mut out);
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn test_case_1() {
        assert_eq!(run("2 4\n"), "4\n");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(run("3 3\n"), "4\n");
    }
}
