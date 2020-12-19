use std::fs;

mod day15;

fn main() {
    // let input = fs::read_to_string("inputs/day15.txt").unwrap();
    println!("part 1: {0}", day15::part_one("0,1,4,13,15,12,16"));
    println!("part 1: {0}", day15::part_two("0,1,4,13,15,12,16"));
}
