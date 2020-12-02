use std::fs;

mod day1;
mod day2;

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    println!("part 1: {0}", day2::part_one(&input));
    println!("part 2: {0}", day2::part_two(&input));
}
