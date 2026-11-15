//! 236A - Problem Name
//!
//! Ref: https://codeforces.com/problemset/problem/236/A

use std::io::{self, BufRead, BufWriter, Write};

pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let n = io::read_to_string(input)
        .unwrap()
        .trim_end()
        .bytes()
        .fold(0u32, |set, c| set | 1 << (c.to_ascii_lowercase() - b'a'))
        .count_ones();

    let m = if n % 2 == 0 {
        "CHAT WITH HER!"
    } else {
        "IGNORE HIM!"
    };
    writeln!(out, "{m}").unwrap();
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
    fn cf_236a_test_case_1() {
        assert_eq!(run("wjmzbmr\n"), "CHAT WITH HER!\n");
    }

    #[test]
    fn cf_236a_test_case_2() {
        assert_eq!(run("xiaodao\n"), "IGNORE HIM!\n");
    }

    #[test]
    fn cf_236a_test_case_3() {
        assert_eq!(run("sevenkplus\n"), "CHAT WITH HER!\n");
    }
}
