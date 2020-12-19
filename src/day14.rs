use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
  set: u64,
  clear: u64,
  floating: u64,
}

#[inline]
fn parse_mask(s: &str) -> Mask {
  let mask_str = s.split(" = ").nth(1).unwrap();
  let mut mask = Mask { set: 0, clear: 0, floating: 0 };

  mask_str.chars()
    .enumerate()
    .for_each(|(index, c)| {
      match c {
        '0' => mask.clear |= 1 << (mask_str.len() - index - 1),
        '1' => mask.set |= 1 << (mask_str.len() - index - 1),
        'X' => mask.floating |= 1 << (mask_str.len() - index - 1),
        _ => (),
      }
    });

  mask.clear = !mask.clear;
  mask
}

#[inline]
fn parse_mem(s: &str) -> (usize, u64) {
  let mut it = s.chars().skip(4); // skip "mem["
  let addr: String = it.by_ref().take_while(|c| c.is_digit(10)).collect();
  it.by_ref().take(3).count(); // skip "] = "
  let num: String = it.by_ref().take_while(|c| c.is_digit(10)).collect();

  (addr.parse::<usize>().unwrap(), num.parse::<u64>().unwrap())
}

pub fn part_one(s: &str) -> u64 {
  let mut memory: HashMap<usize, u64> = HashMap::new();
  let mut mask = parse_mask(s.lines().nth(0).unwrap());
  for line in s.lines().skip(1) {
    if line.starts_with("mask") {
      mask = parse_mask(line);
      continue;
    }
    let instr = parse_mem(line);
    memory.insert(instr.0, (instr.1 & mask.clear) | mask.set);
  }
  memory.iter().fold(0u64, |acc, kvp| acc + kvp.1)
}

// gets the highest bit of a number.
// surely there's a better way of doing this
macro_rules! get_highest_bit {
  ($x:expr) => {
    if $x.is_power_of_two() {
      $x
    } else {
      $x.next_power_of_two() >> 1
    }
  };
}

pub fn part_two(s: &str) -> u64 {
  let mut memory: HashMap<usize, u64> = HashMap::new();
  let mut mask = Mask { set: 0, clear: 0, floating: 0 };
  let mut address_offsets: Vec<u64> = Vec::new();
  for line in s.lines() {
    if line.starts_with("mask") {
      mask = parse_mask(line);
      address_offsets.clear();

      // i think this takes the cake
      // as the worst code i've ever written.
      // well here goes

      // this figures out all the permutations of the floating bits and stores
      // them for later

      // count the bits in the floating mask
      let floating_bit_count = mask.floating.count_ones();

      // 2 ** bit count is the number of possible permutations the floating
      // bits
      let mut i = (2u64).pow(floating_bit_count);

      // the general idae here is this:
      //
      // i counts down to 0, going through every possible permutation of the
      // floating bits. e.g. if 3 and 1 are the floating bits, i tells us which
      // ones to toggle:
      //
      //    i | i bits | floating bits
      //   ---+--------+---------------
      //    0 |     00 |          0000
      //    1 |     01 |          0010
      //    2 |     10 |          1000
      //    3 |     11 |          1010
      //
      //    i bit 0 -> floating bit 1
      //    i bit 1 -> floating bit 3

      while i != 0 {
        i -= 1;

        // keeps track of both the whole floating bitmask & it's highest bit
        let mut floating_remaining = mask.floating;
        let mut highest_bit = get_highest_bit!(floating_remaining);

        let mut j = (2u64).pow(floating_bit_count) >> 1;

        let mut offset = 0;
        // loop over all the floating bits
        loop {
          let bit_set = (i & j) > 0;

          // if the bit is set in i, set the corresponding floating bit in offset
          if bit_set {
            offset |= highest_bit;
          }

          // get the next floating bit
          if highest_bit > 0 {
            floating_remaining &= !highest_bit;
            highest_bit = get_highest_bit!(floating_remaining);
          }
          if j == 0 {
            break;
          }
          j >>= 1;
        }
        address_offsets.push(offset);
      }
      continue;
    }

    let instr = parse_mem(line);
    let base_addr = (instr.0 as u64 | mask.set) & !mask.floating;
    for offset in address_offsets.iter() {
      memory.insert((base_addr | offset) as usize, instr.1);
    }
  }
  memory.iter().fold(0u64, |acc, kvp| acc + kvp.1)
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

static INPUT2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 165);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT2), 208);
  }
}