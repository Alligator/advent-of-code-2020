pub fn part_one(s: &str) -> i64 {
  let cols = s.lines().nth(0).unwrap().len();
  let rows = s.lines().count();

  let get_neighbours = |map: &Vec<char>, idx: usize| {
    let x = (idx % cols) as i64;
    let y = (idx / cols) as i64;
    let mut neighbours: Vec<char> = Vec::new();

    for x_offset in -1..=1 {
      for y_offset in -1..=1 {
        if x_offset == 0 && y_offset == 0 {
          continue;
        }
        if x + x_offset < 0 || x + x_offset >= cols as i64 {
          continue;
        }
        if y + y_offset < 0 || y + y_offset >= rows as i64 {
          continue;
        }
        let index = (x + x_offset) + (y + y_offset) * cols as i64;
        if index >= 0 && index < map.len() as i64 {
          // dbg!(x_offset, y_offset, index);
          neighbours.push(map[index as usize]);
        }
      }
    }

    neighbours
  };

  let mut current: Vec<char> = s.replace("\n", "").chars().collect();
  // print_map(cols, rows, &current);

  loop {
    let mut next_iter: Vec<char> = Vec::new();
    let mut changed = false;
    for i in 0..current.len() {
      let seat = &current[i];
      if *seat == 'L' {
        // If a seat is empty (L) and there are no occupied seats adjacent to it,
        // the seat becomes occupied.
        let neightbours = get_neighbours(&current ,i);
        if neightbours.iter().filter(|s| **s == '#').count() == 0 {
          next_iter.push('#');
          changed = true;
        } else {
          next_iter.push(*seat);
        }
        continue;
      }

      if *seat == '#' {
        // If a seat is occupied (#) and four or more seats adjacent to it are
        // also occupied, the seat becomes empty.
        let neightbours = get_neighbours(&current ,i);
        if neightbours.iter().filter(|s| **s == '#').count() >= 4 {
          next_iter.push('L');
          changed = true;
        } else {
          next_iter.push(*seat);
        }
        continue;
      }

      next_iter.push(*seat);
    }

    current = next_iter;
    // print_map(cols, rows, &current);

    if !changed {
      break;
    }
  }

  let occupied_seats = current.iter().filter(|s| **s == '#').count();
  occupied_seats as i64
}

pub fn part_two(s: &str) -> i64 {
  let cols = s.lines().nth(0).unwrap().len();
  let rows = s.lines().count();

  let get_neighbours = |map: &Vec<char>, idx: usize| {
    let x = (idx % cols) as i64;
    let y = (idx / cols) as i64;
    let mut neighbours: Vec<char> = Vec::new();

    for x_offset in -1..=1 {
      for y_offset in -1..=1 {
        if x_offset == 0 && y_offset == 0 {
          continue;
        }

        let mut ray_x = x + x_offset;
        let mut ray_y = y + y_offset;

        while ray_x >= 0 && ray_x < cols as i64 && ray_y >= 0 && ray_y < rows as i64 {
          let index = ray_x + ray_y * cols as i64;
          if map[index as usize] != '.' {
            neighbours.push(map[index as usize]);
            break;
          }
          ray_x += x_offset;
          ray_y += y_offset;
        }
      }
    }

    neighbours
  };

  let mut current: Vec<char> = s.replace("\n", "").chars().collect();
  // print_map(cols, rows, &current);

  // loop {
  loop {
    let mut next_iter: Vec<char> = Vec::new();
    let mut changed = false;
    for i in 0..current.len() {
      let seat = &current[i];
      if *seat == 'L' {
        // If a seat is empty (L) and there are no occupied seats adjacent to it,
        // the seat becomes occupied.
        let neightbours = get_neighbours(&current ,i);
        if neightbours.iter().filter(|s| **s == '#').count() == 0 {
          next_iter.push('#');
          changed = true;
        } else {
          next_iter.push(*seat);
        }
        continue;
      }

      if *seat == '#' {
        // If a seat is occupied (#) and four or more seats adjacent to it are
        // also occupied, the seat becomes empty.
        let neightbours = get_neighbours(&current ,i);
        // dbg!(i, &neightbours);
        if neightbours.iter().filter(|s| **s == '#').count() >= 5 {
          next_iter.push('L');
          changed = true;
        } else {
          next_iter.push(*seat);
        }
        continue;
      }

      next_iter.push(*seat);
    }

    current = next_iter;

    if !changed {
      break;
    }
  }

  let occupied_seats = current.iter().filter(|s| **s == '#').count();
  occupied_seats as i64
}

fn print_map(cols: usize, rows: usize, map: &Vec<char>) {
  for y in 0..rows {
    for x in 0..cols {
      print!("{}", map[x + y * cols]);
    }
    print!("\n");
  }
  println!("----------");
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 37);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT), 26);
  }
}