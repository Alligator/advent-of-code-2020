use std::collections::HashMap;
fn eval_expr(s: &str, precedence: &HashMap<&str, i64>) -> i64 {
  let src = s
    .replace("(", "( ")
    .replace(")", " )");

  let tokens = src.split(" ");

  // shunting yard time
  let mut output_q: Vec<&str> = Vec::new();
  let mut operator_q: Vec<&str> = Vec::new();

  for token in tokens {
    match token {
      "+" | "*" => {
        while !operator_q.is_empty() {
          let top_op = operator_q[operator_q.len() - 1];
          if top_op == "(" {
            break;
          }
          let op_prec = precedence.get(top_op).unwrap();
          if op_prec >= precedence.get(token).unwrap() {
            output_q.push(operator_q.pop().unwrap());
          } else {
            break;
          }
        }
        operator_q.push(token);
      }
      "(" => {
          operator_q.push(token);
      }
      ")" => {
        while operator_q[operator_q.len() - 1] != "(" {
          output_q.push(operator_q.pop().unwrap());
        }
        if operator_q[operator_q.len() - 1] == "(" {
          operator_q.pop();
        }
      }
      c => {
        let num = c.parse::<i64>();
        if num.is_ok() {
          output_q.push(token);
        } else {
          panic!("unknown token {:?}", c);
        }
      }
    }
    // dbg!(token, &operator_q, &output_q);
  }
  while !operator_q.is_empty() {
    output_q.push(operator_q.pop().unwrap());
  }

  let mut stack = Vec::new();
  for token in output_q.iter() {
    match *token {
      "+" => {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(a + b);
      }
      "*" => {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(a * b);
      }
      c => stack.push(c.parse::<i64>().unwrap()),
    }
  }

  stack[0]
}

pub fn part_one(s: &str) -> i64 {
  let mut precedence = HashMap::new();
  precedence.insert("*", 2);
  precedence.insert("+", 2);
  s.lines().map(|l| eval_expr(l, &precedence)).sum()
}

pub fn part_two(s: &str) -> i64 {
  let mut precedence = HashMap::new();
  precedence.insert("*", 2);
  precedence.insert("+", 3);
  s.lines().map(|l| eval_expr(l, &precedence)).sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("1 + (2 * 3) + (4 * (5 + 6))\n1 + 2 + 3"), 57);
    assert_eq!(part_one("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("1 + (2 * 3) + (4 * (5 + 6))\n1 + 2 + 3"), 57);
    assert_eq!(part_two("2 * 3 + (4 * 5)"), 46);
    assert_eq!(part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
  }
}
