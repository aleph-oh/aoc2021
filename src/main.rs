mod day1;

const DAY: usize = 1;

fn main() {
    if DAY == 1 {
        let (pt1, pt2) = day1::solve();
        println!("day 1 part 1: {} part 2: {}", pt1, pt2);
    }
}
