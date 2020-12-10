pub fn part_one(s: &str, window_size: usize) -> i64 {
  let numbers: Vec<i64> = s.lines().map(|line| line.parse::<i64>().unwrap()).collect();
  'outer:for (index, num) in numbers.iter().enumerate().skip(window_size) {
    let window: Vec<i64> = numbers.iter().cloned()
      .skip(index - window_size)
      .take(window_size)
      .collect();

    for a in window.iter() {
      for b in window.iter() {
        if a != b && a + b == *num {
          continue 'outer;
        }
      }
    }
    // number was invalid
    return *num;
  }
  0
}

pub fn part_two(s: &str, window_size: usize) -> i64 {
  let numbers: Vec<i64> = s.lines().map(|line| line.parse::<i64>().unwrap()).collect();
  let invalid_number = part_one(s, window_size);

  for start_index in 0..numbers.len() {
    let mut sum = numbers[start_index];
    let mut smallest = sum;
    let mut largest = sum;
    for end_index in start_index+1..numbers.len() {
      let num = numbers[end_index];
      if num > largest { largest = num; }
      if num < smallest { smallest = num; }
      sum += num;

      if sum == invalid_number {
        // found it
        return smallest + largest;
      }

      if sum > invalid_number {
        break;
      }
    }
  }
  0
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT, 5), 127);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT, 5), 62);
  }
}