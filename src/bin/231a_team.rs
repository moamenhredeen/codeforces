//! 231A - Team
//!
//! Ref: https://codeforces.com/problemset/problem/231/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let sum: u32 = input
        .lines()
        .skip(1)
        .map(|line| {
            let sum: u32 = line
                .unwrap()
                .split_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .sum();
            if sum > 1 { 1 } else { 0 }
        })
        .sum();
    writeln!(out, "{}", sum).unwrap();
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
        assert_eq!(
            run("3
        1 1 0
        1 1 1
        1 0 0
        "),
            "2\n"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            run("2
        1 0 0
        0 1 1
        "),
            "1\n"
        );
    }
}
