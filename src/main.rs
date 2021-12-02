mod day1;
mod day2;
mod utils;

fn main() {
    let day = std::env::args()
        .skip(1)
        .next()
        .expect("Expected one argument for day")
        .parse::<u8>()
        .expect("Could not parse provided day as u8");
    let (pt1, pt2) = match day {
        1 => day1::solve(),
        2 => day2::solve(),
        _ => panic!(),
    };
    println!("Day {} | Part 1 {} | Part 2 {}", day, pt1, pt2);
}
