use std::collections::HashMap;

fn nth_number(s: &str, n: usize) -> i64 {
  let mut last_seen: HashMap<usize, usize> = HashMap::new();

  let initial_numers: Vec<usize> = s.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
  initial_numers.iter().enumerate().for_each(|(index, n)| {
    last_seen.insert(*n, index);
  });

  let mut prev_number = *initial_numers.last().unwrap();
  for i in initial_numers.len()..n {
    let last = last_seen.insert(prev_number, i - 1);
    prev_number = match last {
      Some(n) => ((i - 1) - n),
      None => 0,
    }
  }
  prev_number as i64
}

pub fn part_one(s: &str) -> i64 {
  nth_number(s, 2020)
}

// well, as long as this is run in release mode it only takes 3 seconds
// in debug mode it takes about 20
pub fn part_two(s: &str) -> i64 {
  nth_number(s, 30000000)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("0,3,6"), 436);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("0,3,6"), 175594);
  }
}