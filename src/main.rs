use std::fs;

mod day9;

fn main() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();
    println!("part 1: {0}", day9::part_one(&input, 25));
    println!("part 2: {0}", day9::part_two(&input, 25));
}
