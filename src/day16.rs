use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Constraint {
  name: String,
  ranges: ((i64, i64), (i64, i64)),
}

type Ticket = Vec<i64>;

#[derive(Debug)]
struct Notes {
  constraints: Vec<Constraint>,
  my_ticket: Ticket,
  nearby_tickets: Vec<Ticket>,
}

fn parse_notes(s: &str) -> Notes {
  enum ParseState { Constraints, MyTicket, NearbyTickets };
  let mut state = ParseState::Constraints;

  let mut notes = Notes {
    constraints: Vec::new(),
    my_ticket: Vec::new(),
    nearby_tickets: Vec::new(),
  };

  for line in s.lines() {
    match line {
      "your ticket:" => {
        state = ParseState::MyTicket;
        continue;
      }
      "nearby tickets:" => {
        state = ParseState::NearbyTickets;
        continue;
      }
      "" => continue,
      _ => (),
    }

    match state {
      ParseState::Constraints => {
        let name = line.split(":").nth(0).unwrap();

        let ranges_str: Vec<&str> = line
          .split(":")
          .nth(1)
          .unwrap()
          .trim()
          .split(" or ")
          .collect();

        let range1: Vec<i64> = ranges_str[0]
          .split("-")
          .map(|s| s.parse::<i64>().unwrap())
          .collect();

        let range2: Vec<i64> = ranges_str[1]
          .split("-")
          .map(|s| s.parse::<i64>().unwrap())
          .collect();

        notes.constraints.push(Constraint {
          name: String::from(name),
          ranges: ((range1[0], range1[1]), (range2[0], range2[1])),
        });
      }
      ParseState::MyTicket => {
        notes.my_ticket = line
          .split(",")
          .map(|s| s.parse::<i64>().unwrap())
          .collect();
      }
      ParseState::NearbyTickets => {
        notes.nearby_tickets.push(line
          .split(",")
          .map(|s| s.parse::<i64>().unwrap())
          .collect()
        );
      }
    }
  }
  notes
}

pub fn part_one(s: &str) -> i64 {
  let notes = parse_notes(s);
  let mut errors: Vec<i64> = Vec::new();

  for ticket in notes.nearby_tickets.iter() {
    let mut ticket_errors: Vec<i64> = ticket.iter()
      .cloned()
      .filter(|num| {
        let mut any_valid = false;
        for constraint in notes.constraints.iter() {
          if (*num >= constraint.ranges.0.0 && *num <= constraint.ranges.0.1)
          || (*num >= constraint.ranges.1.0 && *num <= constraint.ranges.1.1) {
            any_valid = true;
            continue;
          }
        }
        !any_valid
      }).collect();
    if ticket_errors.len() > 0 {
      errors.append(&mut ticket_errors);
    }
  }
  errors.iter().sum()
}

pub fn part_two(s: &str) -> i64 {
  let notes = parse_notes(s);

  // remove invalid tickets
  let valid_tickets = notes.nearby_tickets.iter()
    .cloned()
    .filter(|ticket| {
      ticket.iter()
        .cloned()
        .filter(|num| {
          for constraint in notes.constraints.iter() {
            if (*num >= constraint.ranges.0.0 && *num <= constraint.ranges.0.1)
            || (*num >= constraint.ranges.1.0 && *num <= constraint.ranges.1.1) {
              return true;
            }
          }
          false
        })
        .count() == ticket.len()
    });

  // set up a hashmap of constraint -> possible indexes
  // this is initialised to all possible indexes at first
  let mut poss: HashMap<String, HashSet<i64>> = HashMap::new();
  for constraint in notes.constraints.iter() {
    let mut set = HashSet::new();
    for i in 0..notes.my_ticket.len() {
      set.insert(i as i64);
    }
    poss.insert(String::from(&constraint.name), set);
  }

  for ticket in valid_tickets {
    for (index, num) in ticket.iter().enumerate() {
      for constraint in notes.constraints.iter() {
        let set = poss.get_mut(&constraint.name).unwrap();
        // if a value at an index is not value for a constraint
        // remove it from from the possibilities
        if !((*num >= constraint.ranges.0.0 && *num <= constraint.ranges.0.1)
        || (*num >= constraint.ranges.1.0 && *num <= constraint.ranges.1.1)) {
          set.remove(&(index as i64));
        }
      }
    }
  }

  let mut done = false;
  let mut final_mappings: HashMap<String, i64> = HashMap::new();

  while !done {
    done = false;
    // find the unambiguous one(s)
    let mut unambig: HashMap<String, i64> = HashMap::new();
    for (k, v) in poss.iter() {
      if v.len() == 1 {
        unambig.insert(k.to_string(), *v.iter().nth(0).unwrap());
      }
    }

    // insert them into final_mappings
    for (name, val) in unambig.iter() {
      final_mappings.insert(name.to_string(), *val);
    }

    // remove them from the other things
    let values: Vec<i64> = unambig.iter().map(|(_, v)| *v).collect();
    for (_, val) in poss.iter_mut() {
      for v in values.iter() {
        val.remove(v);
      }
    }

    if final_mappings.len() == notes.constraints.len() {
      done = true;
    }
  }

  final_mappings.iter()
    .filter(|(k, _)| k.starts_with("departure"))
    .fold(1, |acc, (_, v)| notes.my_ticket[*v as usize] * acc)
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

  static INPUT2: &str = "departure class: 0-1 or 4-19
row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 71);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT2), 12 * 13);
  }
}


