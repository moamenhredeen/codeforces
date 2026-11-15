//! 282A - Bit++
//!
//! Ref: https://codeforces.com/problemset/problem/282/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let n = input.lines().skip(1).fold(0, |acc, line| {
        let operation = line.unwrap();
        if operation.contains("+") {
            acc + 1
        } else {
            acc - 1
        }
    });
    writeln!(out, "{n}").unwrap();
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
        assert_eq!(run("1\n++X\n"), "1\n");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(run("2\nX++\n--X\n"), "0\n");
    }
}
