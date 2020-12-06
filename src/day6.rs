use std::collections::HashSet;

pub fn part_one(s: &str) -> i64 {
  s.split("\n\n")
    .map(|group| {
      let answers: HashSet<char> = group
        .chars()
        .filter(|c| *c != '\n')
        .collect();
      answers.len()
    })
    .fold(0, |acc, v| acc + v) as i64
}

pub fn part_two(s: &str) -> i64 {
  s.split("\n\n")
    .map(|group| {
      let lines: Vec<&str> = group.lines().collect();
      let mut answers: HashSet<char> = lines[0]
        .chars()
        .collect();

      for line in &lines[1..] {
        let a: HashSet<char> = line.chars().collect();
        answers = answers.intersection(&a).cloned().collect();
      }
      answers.len()
    })
    .fold(0, |acc, v| acc + v) as i64
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("abc

a
b
c

ab
ac

a
a
a
a

b"), 11);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("abc

a
b
c

ab
ac

a
a
a
a

b"), 6);
  }
}
