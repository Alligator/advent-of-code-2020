use std::fs;

mod day12;

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    println!("part 1: {0}", day12::part_one(&input));
    println!("part 2: {0}", day12::part_two(&input));
}
