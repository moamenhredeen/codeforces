//! 71A - Way Too Long Words
//!
//! Ref: https://codeforces.com/problemset/problem/71/A

//! 71A - Way Too Long Words
//!
//! Ref: https://codeforces.com/problemset/problem/71/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    for line in input.lines().skip(1) {
        let word = line.unwrap();
        let len = word.len();
        if len > 10 {
            writeln!(out, "{}{}{}", &word[..1], len - 2, &word[len - 1..]).unwrap();
        } else {
            writeln!(out, "{}", word).unwrap();
        }
    }
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
        assert_eq!(run("1\nabc\n"), "abc\n");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(run("2\nabc\nabc\n"), "abc\nabc\n");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(run("3\nabc\nabc\nabcabcabcabc\n"), "abc\nabc\na10c\n");
    }
}
