use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Rule {
  container: String,
  contents: HashMap<String, i64>,
}

macro_rules! expect {
  ($iter:expr, $x:literal) => {
    if ($iter.by_ref().next().unwrap() != $x) {
      panic!(format!("expected {0}", $x));
    }
  };
}

impl Rule {
  fn from_str(s: &str) -> Option<Self> {
    let mut iter = s.split_ascii_whitespace();

    let container: String = iter.by_ref().take(2).collect();
    expect!(iter, "bags");
    expect!(iter, "contain");

    let mut contents: HashMap<String, i64> = HashMap::new();

    loop {
      let num = iter.by_ref().next().unwrap();
      if num == "no" {
        break;
      }

      let child_container: String = iter.by_ref().take(2).collect();
      contents.insert(child_container, num.parse::<i64>().unwrap());

      if iter.next().unwrap().ends_with(".") {
        break;
      }
    }

    Some(Rule {
      container,
      contents,
    })
  }
}

fn find_containers(colour: &str, rules: Vec<Rule>) -> Vec<Rule> {
  let mut containers: HashMap<&str, Rule> = HashMap::new();
  let mut current_search: HashSet<&str> = HashSet::new();

  current_search.insert(colour);

  let mut found = true;
  while found {
    found = false;
    let mut next_search: HashSet<&str> = HashSet::new();

    for rule in rules.iter() {
      if rule.contents.keys().any(|r| current_search.contains(r.as_str())) {
        containers.insert(rule.container.as_str(), rule.clone());
        next_search.insert(rule.container.as_str());
        found = true;
      }
    }

    current_search = next_search;
  }

  containers.values().cloned().collect()
}

fn sum_bags(colour: &str, rules: Vec<Rule>) -> i64 {
  let mut sum_cache: HashMap<String, i64> = HashMap::new();

  fn sum_bag (bag_colour: &str, rules: &Vec<Rule>, sum_cache: &mut HashMap<String, i64>) -> i64 {
    if sum_cache.contains_key(bag_colour) {
      return *sum_cache.get(bag_colour).unwrap();
    }

    let rule = rules.iter()
      .find(|rule| rule.container == bag_colour)
      .unwrap();

    let mut sum = 1;
    for child in rule.contents.iter() {
      sum += sum_bag(child.0.as_str(), rules, sum_cache) * child.1;
    }
    sum_cache.insert(String::from(bag_colour), sum);
    sum
  }
  sum_bag(colour, &rules, &mut sum_cache) - 1
}

pub fn part_one(s: &str) -> i64 {
  let rules: Vec<Rule> = s.lines().map(|l| Rule::from_str(l).unwrap()).collect();
  find_containers("shinygold", rules).len() as i64
}

pub fn part_two(s: &str) -> i64 {
  let rules: Vec<Rule> = s.lines().map(|l| Rule::from_str(l).unwrap()).collect();
  sum_bags("shinygold", rules)
}

#[cfg(test)]
mod test {
  use super::*;

  static INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

  #[test]
  fn test_part_one() {
    assert_eq!(part_one(INPUT), 4);
  }

  #[test]
  fn test_part_two() {
    assert_eq!(part_two(INPUT), 32);
  }
}