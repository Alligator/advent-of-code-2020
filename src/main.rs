use std::fs;

mod day6;

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    println!("part 1: {0}", day6::part_one(&input));
    println!("part 2: {0}", day6::part_two(&input));
}
