use crate::utils::input;

fn part_one(sorted_inputs: &[u64]) -> i64 {
    let n = sorted_inputs.len();
    let median = {
        if sorted_inputs.len() % 2 == 0 {
            ((sorted_inputs[n / 2 - 1] + sorted_inputs[n / 2]) as f64 / 2f64) as u64
        } else {
            sorted_inputs[n / 2]
        }
    };
    sorted_inputs
        .iter()
        .map(|&num| (num as i64 - median as i64).abs() as u64)
        .sum::<u64>() as i64
}

fn part_two(inputs: &[u64]) -> i64 {
    fn distance_metric(a: u64, b: u64) -> u64 {
        let abs_diff = (a as i64 - b as i64).abs() as u64;
        (abs_diff * (abs_diff + 1)) / 2
    }
    let min = *inputs.iter().min().expect("Expected at least one element");
    let max = *inputs.iter().max().expect("Expected at least one element");
    (min..=max)
        .map(|n| inputs.iter().map(|&m| distance_metric(n, m)).sum::<u64>())
        .min()
        .expect("Expected at least one element") as i64
}

pub(crate) fn solve() -> (i64, i64) {
    let inputs = input();
    let mut parsed_inputs = inputs
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .expect("Failed to parse input...");
    parsed_inputs.sort_unstable();
    let parsed_inputs = parsed_inputs;
    (part_one(&parsed_inputs), part_two(&parsed_inputs))
}
