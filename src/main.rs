use std::fs;

mod day4;

fn main() {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    println!("part 1: {0}", day4::part_one(&input));
    println!("part 2: {0}", day4::part_two(&input));
}
