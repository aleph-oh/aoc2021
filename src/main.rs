#![feature(array_zip)]

mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Expected one argument for day")
        .parse::<u8>()
        .expect("Could not parse provided day as u8");
    let (pt1, pt2) = match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => panic!(),
    };
    println!("Day {} | Part 1 {} | Part 2 {}", day, pt1, pt2);
}
