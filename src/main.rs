use std::fs;

// mod day13;
mod day14;

fn main() {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    println!("part 1: {0}", day14::part_one(&input));
    println!("part 2: {0}", day14::part_two(&input));
}
