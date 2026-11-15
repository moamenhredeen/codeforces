# Kata

Competitive programming solutions in Rust.

## Layout

One file per problem in `src/bin/`. Cargo picks up each file as its own binary,
so nothing needs to be registered in `Cargo.toml`.

```
src/bin/cf_263a_beautiful_matrix.rs
        │  │    └ problem name
        │  └ contest number + problem letter
        └ platform (cf = Codeforces)
```

Every file is self-contained and uses only `std`, so it can be submitted as is.

## Usage

```sh
cargo test --bin cf_263a_beautiful_matrix          # run the sample tests
cargo run --quiet --bin cf_263a_beautiful_matrix   # run against stdin
```

Both are also available as editor tasks for the currently open file
(`.zed/tasks.json`, `.vscode/tasks.json`).

Avoid a bare `cargo test`: it compiles every problem.

## Starting a new problem

1. Create `src/bin/cf_<contest><letter>_<name>.rs`.
2. Expand the `cf` snippet: `solve(input, out)`, `main` and a test module.
   In VS Code the header and problem URL are filled in from the file name.
3. Add the sample cases with the `cft` snippet.

`solve` is generic over `BufRead` and `Write`, so tests feed it a string and
compare the output without touching stdin or stdout.

## Resources

### [USACO Guide](https://usaco.guide/dashboard)

A free, community-maintained curriculum, not a judge. Topics are ordered along
the USACO divisions (Bronze, Silver, Gold, Platinum); each module explains one
technique and links a curated set of problems from other sites, with progress
tracking.

Good for: learning techniques in a sensible order and knowing what to study
next.

### [Codeforces](https://codeforces.com/)

The largest competitive programming community, with several rated rounds a
week split into divisions (Div. 4 easiest, Div. 1 hardest). Every problem in
the archive carries a difficulty rating from 800 to 3500, and almost all have
an editorial. Easier problems lean towards ad-hoc reasoning, math and
observations rather than textbook algorithms. Rust submissions are limited to
`std`.

Good for: live contest practice, speed, and targeted practice by filtering the
problem set to a rating slightly above the current level.

### [AtCoder](https://atcoder.jp/)

A Japanese platform with weekend contests in three tiers: Beginner (ABC),
Regular (ARC) and Grand (AGC). Statements are short and precise, and ABC
problems climb steadily in difficulty from A onwards while covering standard
techniques. Rust submissions may use a fixed set of crates such as `proconio`
and `itertools`.

Good for: building fundamentals on well-made problems with a smooth difficulty
ramp.
