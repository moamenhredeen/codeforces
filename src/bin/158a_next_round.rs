//! 158A - Next Round
//!
//! Ref: https://codeforces.com/problemset/problem/158/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let mut lines = input.lines().map(|line| line.unwrap());
    let nk: Vec<u32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    let particpants: Vec<u32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    let mut res = 0;
    for p in &particpants {
        if p > &0 && p >= &particpants[(nk[1] - 1) as usize] {
            res += 1
        }
    }
    writeln!(out, "{res}").unwrap();
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
        assert_eq!(run("8 5\n10 9 8 7 7 7 5 5\n"), "6\n");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(run("4 2\n0 0 0 0\n"), "0\n");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(run("5 5\n1 1 1 1 1\n"), "5\n");
    }
}
