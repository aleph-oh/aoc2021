use crate::utils::input;
use std::collections::HashMap;

fn simulate(inputs: [u64; 9], n_days: u16) -> i64 {
    let mut all_fish: [u64; 9] = inputs;
    for _ in 0..n_days {
        let to_reset = all_fish[0];
        for i in 1..=8 {
            all_fish[i - 1] = all_fish[i];
        }
        all_fish[8] = to_reset;
        all_fish[6] += to_reset;
    }
    all_fish.iter().sum::<u64>() as i64
}

pub(crate) fn solve() -> (i64, i64) {
    let inputs = input();
    let parsed_inputs = inputs
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u8>, _>>()
        .expect("Failed to parse input...");
    let frequencies: HashMap<u8, u64> =
        parsed_inputs
            .into_iter()
            .fold(HashMap::new(), |mut map, x| {
                *map.entry(x).or_default() += 1;
                map
            });
    let mut fish_frequencies = [0u64; 9];
    for (i, n) in frequencies {
        fish_frequencies[i as usize] = n;
    }
    (
        simulate(fish_frequencies, 80),
        simulate(fish_frequencies, 256),
    )
}
