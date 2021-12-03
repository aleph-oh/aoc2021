use crate::utils::input;
use std::ops::Not;

const MAX_NUM_BITS: usize = 12;

type BitArray = [bool; MAX_NUM_BITS];

fn vector_bits_set(v: &[u16]) -> [u64; MAX_NUM_BITS] {
    v.iter().fold([0u64; MAX_NUM_BITS], |mut acc, &n| {
        for (i, item) in acc.iter_mut().enumerate().take(MAX_NUM_BITS) {
            let b = ((n & (1 << i)) >> i) as u64;
            assert!((b == 0) | (b == 1), "{}", b);
            *item += b;
        }
        acc
    })
}

fn vector_ones_set_ge(v: &[u16]) -> BitArray {
    let bits_set = vector_bits_set(v);
    assert!(
        bits_set.iter().all(|&n| n <= v.len() as u64),
        "{:?}",
        bits_set
    );
    bits_set.map(|n| n >= (v.len() / 2) as u64)
}

fn num_from_bitarray(arr: &BitArray) -> u64 {
    (0..MAX_NUM_BITS)
        .into_iter()
        .fold(0u64, |acc, i| acc | ((arr[i] as u64) << i))
}

fn part_one(nums: &[u16]) -> i64 {
    let gamma_array = vector_ones_set_ge(nums);
    let gamma_rate = num_from_bitarray(&gamma_array) as i64;
    let epsilon_array = gamma_array.map(bool::not);
    let epsilon_rate = num_from_bitarray(&epsilon_array) as i64;
    gamma_rate * epsilon_rate
}

const fn get_bit(n: u16, i: u8) -> u16 {
    (n & 1 << i) >> i
}

const fn reverse_k_bit_number(n: u16, k: u8) -> u16 {
    let shift_amt = 16 - k as u16;
    n.reverse_bits() >> shift_amt
}

fn most_common_value_in_pos(nums: &[u16], i: u8) -> u16 {
    assert!(nums.iter().all(|n| n.count_ones() as usize <= MAX_NUM_BITS));
    let set_bit_count = nums.iter().map(|&n| get_bit(n, i)).sum::<u16>() as usize;
    (2 * set_bit_count >= nums.len()) as u16
}

fn least_common_value_in_pos(nums: &[u16], i: u8) -> u16 {
    (most_common_value_in_pos(nums, i) == 0) as u16
}

fn o2_rating(nums: &[u16]) -> i64 {
    let mut all_nums = nums.to_vec();
    let mut i: u8 = 0;
    while all_nums.len() > 1 {
        let mcv_in_pos = most_common_value_in_pos(&all_nums, i);
        all_nums.retain(|n| get_bit(*n, i) == mcv_in_pos);
        i += 1;
    }
    assert_eq!(all_nums.len(), 1);
    reverse_k_bit_number(all_nums[0], MAX_NUM_BITS as u8) as i64
}

fn co2_rating(nums: &[u16]) -> i64 {
    let mut all_nums = nums.to_vec();
    let mut i: u8 = 0;
    while all_nums.len() > 1 {
        let lcv_in_pos = least_common_value_in_pos(&all_nums, i);
        all_nums.retain(|n| get_bit(*n, i) == lcv_in_pos);
        i += 1;
    }
    assert_eq!(all_nums.len(), 1);
    reverse_k_bit_number(all_nums[0], MAX_NUM_BITS as u8) as i64
}

fn part_two(nums: &[u16]) -> i64 {
    let nums: Vec<u16> = nums
        .iter()
        .copied()
        .map(|n| reverse_k_bit_number(n, MAX_NUM_BITS as u8))
        .collect();
    // We reverse here so we can access bits without subtracting from the width of a number - 1.
    o2_rating(&nums) * co2_rating(&nums)
}

pub(crate) fn solve() -> (i64, i64) {
    let inputs: Vec<u16> = input()
        .lines()
        .into_iter()
        .map(|s| u16::from_str_radix(s, 2).expect("Couldn't parse as u16"))
        .collect();
    (part_one(&inputs), part_two(&inputs))
}
