use std::fs;

mod day18;

fn main() {
    let input = fs::read_to_string("inputs/day18.txt").unwrap();
    println!("part 1: {0}", day18::part_one(&input));
    println!("part 2: {0}", day18::part_two(&input));
}
