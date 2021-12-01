use crate::utils::{input, parse_input};

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
    let v = parse_input(&input());
    (num_increases(&v), num_k_window_increases(&v, 3))
}
