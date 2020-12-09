use std::fs;

mod day7;

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    println!("part 1: {0}", day7::part_one(&input));
    println!("part 2: {0}", day7::part_two(&input));
}
