use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
  Nop,
  Acc,
  Jmp,
}

#[derive(Debug, Clone)]
struct Instruction {
  op_code: OpCode,
  operand: i64,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
  s.lines()
    .map(|line| {
      let sp: Vec<&str> = line.split(" ").collect();
      let num = sp[1].parse::<i64>().unwrap();
      let op_code = match sp[0] {
        "nop" => OpCode::Nop,
        "acc" => OpCode::Acc,
        "jmp" => OpCode::Jmp,
        _ => panic!("unknown opcode {0}", sp[0]),
      };
      Instruction { op_code, operand: num }
    })
    .collect()
}

#[derive(Debug)]
struct EvalResult {
  acc: i64,
  terminated : bool,
  visited_addresses: HashSet<i64>,
}

fn eval(program: &Vec<Instruction>, start_pc: i64) -> EvalResult {
  let mut visited_addresses: HashSet<i64> = HashSet::new();
  let mut pc: i64 = start_pc;
  let mut acc: i64 = 0;
  let terminated: bool;

  loop {
    if visited_addresses.contains(&pc) {
      // infinite loop
      terminated = false;
      break;
    }
    visited_addresses.insert(pc);

    if pc as usize >= program.len() {
      terminated = true;
      break;
    }

    let instr = &program[pc as usize];
    match instr.op_code {
      OpCode::Nop => {
        pc += 1;
      }
      OpCode::Acc => {
        acc += instr.operand;
        pc += 1;
      }
      OpCode::Jmp => pc += instr.operand,
    }
  }
  EvalResult {
    acc,
    terminated,
    visited_addresses,
  }
}


pub fn part_one(s: &str) -> i64 {
  let instructions = parse_instructions(s);
  eval(&instructions, 0).acc
}

pub fn part_two(s: &str) -> i64 {
  let instructions = parse_instructions(s);

  // initial run, collect candidates
  let result = eval(&instructions, 0);
  let candidates: Vec<i64> = result.visited_addresses
    .iter().cloned()
    .filter(|addr| match instructions[*addr as usize].op_code {
      OpCode::Nop => true,
      OpCode::Jmp => true,
      _ => false,
    })
    .collect();

    for candidate in candidates {
      let new_program: Vec<Instruction> = instructions
        .iter().cloned()
        .enumerate()
        .map(|(idx, instr)| {
          if idx == candidate as usize {
            return Instruction {
              op_code: match instr.op_code {
                OpCode::Nop => OpCode::Jmp,
                OpCode::Jmp => OpCode::Nop,
                OpCode::Acc => OpCode::Acc,
              },
              operand: instr.operand,
            };
          }
          instr
        })
        .collect();

      let result = eval(&new_program, 0);
      if result.terminated {
        return result.acc;
      }
    }
  0
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 5);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT), 8);
  }
}