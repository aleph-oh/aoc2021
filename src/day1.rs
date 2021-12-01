use std::io::{self, BufRead};

fn parse_input() -> Vec<i64> {
    // This works, but uses linear extra space. You can do it in constant extra space.
    let stdin = io::stdin();
    let mut results = Vec::new();
    let mut iter = stdin.lock().lines();
    while let Some(n) = iter
        .next()
        .and_then(|line| line.expect("Failed to get line").parse().ok())
    {
        results.push(n);
    }
    results
}

fn num_increases(xs: &[i64]) -> usize {
    // Special case w/ windows where we want to see adjacent pairs -> size-1 adjacent windows
    num_k_window_increases(xs, 1)
}

fn num_k_window_increases(xs: &[i64], k: usize) -> usize {
    assert_ne!(k, 0, "nonsensical for size-0 window case");
    // This works because size-k adjacent windows will only differ in one:
    // over [a, b, c, d] with size-3 windows, we must only compare a and d, as b and c are in both.
    xs.windows(k + 1)
        .map(|sub| (sub[0] < sub[k]) as usize)
        .sum()
}

pub fn solve() -> (usize, usize) {
    let v = parse_input();
    (num_increases(&v), num_k_window_increases(&v, 3))
}
