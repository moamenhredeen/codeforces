//! 112A - Petya
//!
//! Ref: https://codeforces.com/problemset/problem/112/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let mut lines = input.lines().map(|line| line.unwrap().to_ascii_lowercase());
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();
    let res = first.cmp(&second) as i32;
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
    fn cf_112a_test_case_1() {
        assert_eq!(run("abc\nabc\n"), "0\n");
    }

    #[test]
    fn cf_112a_test_case_2() {
        assert_eq!(run("abc\nabC\n"), "0\n");
    }

    #[test]
    fn cf_112a_test_case_3() {
        assert_eq!(run("abc\nabD\n"), "-1\n");
    }

    #[test]
    fn cf_112a_test_case_4() {
        assert_eq!(run("abd\nabc\n"), "1\n");
    }

    #[test]
    fn cf_112a_test_case_5() {
        assert_eq!(run("aslkjlkasdd\nasdlkjdajwi\n"), "1\n");
    }
}
