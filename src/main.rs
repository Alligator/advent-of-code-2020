use std::fs;

mod day11;

fn main() {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();
    println!("part 1: {0}", day11::part_one(&input));
    println!("part 2: {0}", day11::part_two(&input));
}
