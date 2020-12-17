#[derive(Debug)]
struct Ship {
  dir: f64,
  east: i64,
  north: i64,
}

pub fn part_one(s: &str) -> i64 {
  let mut ship = Ship { dir: 0.0, east: 0, north: 0 };
  for instr in s.lines() {
    let num = instr[1..].parse::<i64>().unwrap();
    let op = instr.chars().nth(0);
    match op {
      Some('N') => ship.north += num,
      Some('S') => ship.north -= num,
      Some('E') => ship.east += num,
      Some('W') => ship.east -= num,
      Some('F') => {
        ship.east += ship.dir.to_radians().cos() as i64 * num;
        ship.north -= ship.dir.to_radians().sin() as i64 * num;
      }
      Some('R') => ship.dir = (ship.dir + (num as f64)) % 360.0,
      Some('L') => ship.dir = (ship.dir - (num as f64)) % 360.0,
      Some(o) => panic!("unknown opcode {}", o),
      _ => panic!("aaa"),
    }
  }
  ship.east.abs() + ship.north.abs()
}

pub fn part_two(s: &str) -> i64 {
  let mut ship = Ship { dir: 0.0, east: 0, north: 0 };
  let mut waypoint = (10, 1);
  for instr in s.lines() {
    let num = instr[1..].parse::<i64>().unwrap();
    let op = instr.chars().nth(0);
    match op {
      Some('N') => waypoint.1 += num,
      Some('S') => waypoint.1 -= num,
      Some('E') => waypoint.0 += num,
      Some('W') => waypoint.0 -= num,
      Some('F') => {
        ship.east += waypoint.0 * num;
        ship.north += waypoint.1 * num;
      }
      Some('R') | Some('L') => {
        let mut fnum = num as f64;
        if op.unwrap() == 'R' {
          fnum = -fnum;
        }
        let cos = fnum.to_radians().cos() as i64;
        let sin = fnum.to_radians().sin() as i64;
        let x = waypoint.0 * cos - waypoint.1 * sin;
        let y = waypoint.0 * sin + waypoint.1 * cos;
        waypoint.0 = x;
        waypoint.1 = y;
      }
      Some(o) => panic!("unknown opcode {}", o),
      _ => panic!("aaa"),
    }
  }
  ship.east.abs() + ship.north.abs()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("F10
N3
F7
R90
F11"), 25);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("F10
N3
F7
R90
F11"), 286);
  }
}