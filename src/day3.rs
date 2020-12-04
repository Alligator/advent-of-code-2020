struct Map {
  height: usize,
  width: usize,
  map: Vec<char>
}

impl Map {
  pub fn from_str(s: &str) -> Map {
    let height = s.lines().count() - 1;
    let width = s.lines().nth(0).unwrap().chars().count();
    let map: Vec<char> = s.replace("\n", "").chars().collect();
    Map {
      height,
      width,
      map,
    }
  }

  pub fn count_trees_for_slope(&self, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    while y < self.height {
      x = (x + right) % self.width;
      y += down;

      let idx = x + y * self.width;
      if self.map[idx] == '#' {
        count += 1;
      }
    }

    count
  }
}

pub fn part_one(s: &str) -> usize {
  let map = Map::from_str(s);
  map.count_trees_for_slope(3, 1)
}

pub fn part_two(s: &str) -> usize {
  let map = Map::from_str(s);
  map.count_trees_for_slope(1, 1)
  * map.count_trees_for_slope(3, 1)
  * map.count_trees_for_slope(5, 1)
  * map.count_trees_for_slope(7, 1)
  * map.count_trees_for_slope(1, 2)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"), 7);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"), 336);
  }
}