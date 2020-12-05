use std::collections::HashSet;

pub fn decode(s: &str) -> (i64, i64) {
  let mut start_row = 0;
  let mut end_row = 127;

  let mut start_col = 0;
  let mut end_col = 7;

  for c in s.chars() {
    match c {
      'F' => end_row -= (end_row - start_row + 1) / 2,
      'B' => start_row += (end_row - start_row + 1) / 2,
      'R' => start_col += (end_col - start_col + 1) / 2,
      'L' => end_col -= (end_col - start_col + 1) / 2,
      _ => (),
    }
  }

  if start_row != end_row {
    panic!("rows don't match!");
  }
  if start_col != end_col {
    panic!("cols don't match!");
  }

  (start_row, start_col)
}

pub fn part_one(s: &str) -> i64 {
  s.lines()
    .map(|pass| {
      let p = decode(pass);
      p.0 * 8 + p.1
    })
    .max()
    .unwrap()
}

pub fn part_two(s: &str) -> i64 {
  // create a set of all seats
  let mut seats = HashSet::new();
  for row in 0..=127 {
    for col in 0..=7 {
      seats.insert((row, col));
    }
  }

  // remove populated seats and store ids
  let mut ids = HashSet::new();
  for line in s.lines() {
    let seat = decode(line);
    seats.remove(&seat);
    ids.insert(seat.0 * 8 + seat.1);
  }

  // find seats with id + 1 and id - 1 jn the list
  let r: Vec<&(i64, i64)> = seats.iter().filter(|seat| {
    let id = seat.0 * 8 + seat.1;
    ids.contains(&(id + 1)) && ids.contains(&(id - 1))
  }).collect();

  if r.len() != 1 {
    panic!("> 1 seat found!");
  }

  r[0].0 * 8 + r[0].1
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_decode() {
    assert_eq!(part_one("FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR"), 567);
  }
}
