//! 4A - Watermelon
//!
//! Ref: https://codeforces.com/problemset/problem/4/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(mut input: R, mut out: W) {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let w: u32 = s.trim().parse().unwrap();
    let answer = if w % 2 == 0 && w > 2 { "YES" } else { "NO" };
    writeln!(out, "{answer}").unwrap();
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
        assert_eq!(run("8\n"), "YES\n");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(run("2\n"), "NO\n");
    }
}
