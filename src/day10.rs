pub fn part_one(s: &str) -> i64 {
  let mut numbers: Vec<i64> = s.lines()
    .map(|line| line.parse::<i64>().unwrap())
    .collect();
  numbers.sort();

  let mut prev = 0;
  let mut one_diffs = 0;
  let mut three_diffs = 1;

  for number in numbers.iter() {
    let diff = number - prev;
    // dbg!(number, diff);
    if diff == 1 { one_diffs += 1; }
    if diff == 3 { three_diffs += 1; }
    prev = *number;
  }
  one_diffs * three_diffs
}

pub fn part_two(s: &str) -> i64 {
  let mut numbers: Vec<i64> = s.lines()
    .map(|line| line.parse::<i64>().unwrap())
    .collect();
  numbers.sort();
  numbers.push(numbers.last().unwrap() + 3);
  numbers.insert(0, 0);

  let mut count = 1;
  let mut index = 0;
  let mut run_size = 0;

  for number in numbers[..numbers.len()-1].iter() {
    let diff = numbers[index + 1] - number;

    if diff == 1 {
      run_size += 1;
    } else if run_size > 1 {
      let mut pow = (2 as i64).pow(run_size - 2);
      pow = pow + pow / 2;
      count *= pow + 1;
      run_size = 0;
    } else {
      run_size = 0;
    }

    index += 1;
  }
  count
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

  static INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 7 * 5);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT), 8);
    assert_eq!(part_two(INPUT2), 19208);
  }
}