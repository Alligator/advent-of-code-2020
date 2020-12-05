use std::fs;

mod day5;

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    println!("part 1: {0}", day5::part_one(&input));
    println!("part 2: {0}", day5::part_two(&input));
}
