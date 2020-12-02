use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    println!("part 1: {0}", day1::part_one(&input));
    println!("part 2: {0}", day1::part_two(&input));
}
