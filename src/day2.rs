use std::str::FromStr;

#[derive(Debug)]
struct Policy {
  lowest: i64,
  highest: i64,
  letter: char,
}

impl FromStr for Policy {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut iter = s.chars();

    let first: String = iter.by_ref().take_while(|c| c.is_numeric()).collect();
    let second: String = iter.by_ref().take_while(|c| c.is_numeric()).collect();

    let first_num = first.parse::<i64>().unwrap();
    let second_num = second.parse::<i64>().unwrap();
    let letter = iter.next().unwrap();

    Ok(Policy {
      lowest: first_num,
      highest: second_num,
      letter,
    })
  }
}

fn parse_password(s: &str) -> (Policy, &str) {
  let policy = Policy::from_str(s).unwrap();
  let password = s.split(':').last().unwrap().trim();

  (policy, password)
}

fn validate_password_p1(s: &str) -> bool {
  let (policy, password) = parse_password(s);
  let count = password.chars().filter(|c| *c == policy.letter).count() as i64;
  count >= policy.lowest && count <= policy.highest
}

fn validate_password_p2(s: &str) -> bool {
  let (policy, password) = parse_password(s);
  let first_matches = password.chars().nth(policy.lowest as usize - 1).unwrap() == policy.letter;
  let second_matches = password.chars().nth(policy.highest as usize - 1).unwrap() == policy.letter;
  (first_matches && !second_matches) || (!first_matches && second_matches)
}

pub fn part_one(s: &str) -> i64 {
  s.lines().filter(|s| validate_password_p1(s)).count() as i64
}

pub fn part_two(s: &str) -> i64 {
  s.lines().filter(|s| validate_password_p2(s)).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
      assert_eq!(part_one("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn test_part_two() {
      assert_eq!(part_two("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}