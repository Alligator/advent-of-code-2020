use std::fs;

mod day16;

fn main() {
    let input = fs::read_to_string("inputs/day16.txt").unwrap();
    println!("part 1: {0}", day16::part_one(&input));
    println!("part 2: {0}", day16::part_two(&input));
}
