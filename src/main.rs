use std::fs;

mod day10;

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").unwrap();
    println!("part 1: {0}", day10::part_one(&input));
    println!("part 2: {0}", day10::part_two(&input));
}
