//! 263A - Beautiful Matrix
//!
//! Ref: https://codeforces.com/problemset/problem/263/A

use std::io::{self, BufRead, BufWriter, Write};

/// # Solution 1
///
///  0   1   [2]   3   4
/// +1  +1    -   +1  +1
///  x   .    .    .   .
///  2 - 0  = 2
///  .   .    .    x   .
///  2 - 3 = -1
// pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
//     let (x, y) = input
//         .lines()
//         .map(|e| e.unwrap())
//         .enumerate()
//         .find_map(|(i, row)| {
//             row.split_ascii_whitespace()
//                 .position(|c| c == "1")
//                 .map(|j| (i as i32, j as i32))
//         })
//         .unwrap();
//     let moves = (2 - x).abs() + (2 - y).abs();
//     writeln!(out, "{}", moves).unwrap();
// }

/// Solution 2
/// 0 1 2 ... 24
/// . . 1 ... .
/// idx = 2
/// row = idx / 5 = 0 -> row abs_diff 2
/// col = idx % 5 = 2 -> col abs_diff 2
pub fn solve<R: BufRead, W: Write>(input: R, mut out: W) {
    let s = io::read_to_string(input).unwrap();
    let idx = s.split_ascii_whitespace().position(|c| c == "1").unwrap();
    let moves = (idx / 5).abs_diff(2) + (idx % 5).abs_diff(2);
    writeln!(out, "{}", moves).unwrap();
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
    fn cf_263a_test_case_1() {
        assert_eq!(
            run("\
            0s0 0 0 0
            0 0 0 0 1
            0 0 0 0 0
            0 0 0 0 0
            0 0 0 0 0
            "),
            "3\n"
        );
    }

    #[test]
    fn cf_263a_test_case_2() {
        assert_eq!(
            run("\
            0 0 0 0 0
            0 0 0 0 0
            0 1 0 0 0
            0 0 0 0 0
            0 0 0 0 0
            "),
            "1\n"
        );
    }
}
