fn parse_numbers(num_str: &str) -> Vec<i64> {
    return num_str
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

pub fn part_one(num_str: &str) -> i64 {
    let nums = parse_numbers(num_str);
    for num in nums.iter() {
        for num2 in nums.iter() {
            if num + num2 == 2020 {
                return num * num2;
            }
        }
    }

    return 0;
}
pub fn part_two(num_str: &str) -> i64 {
    let nums = parse_numbers(num_str);
    for num in nums.iter() {
        for num2 in nums.iter() {
            if num + num >= 2020 {
                continue;
            }

            for num3 in nums.iter() {
                if num + num2 + num3 == 2020 {
                    return num * num2 * num3;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1721\n979\n366\n299\n675\n1456"), 514579);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1721\n979\n366\n299\n675\n1456"), 241861950);
    }
}