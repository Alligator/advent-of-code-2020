use std::iter::{Peekable};
use std::str::{Chars};

pub fn part_one(s: &str) -> i64 {
  let r = s.split("\n\n")
    .map(|kvp| kvp
      .split(&[' ', '\n'][..])
      .filter(|s| !s.contains("cid"))
      .count())
    .filter(|count| *count == 7)
    .count();
  return r as i64;
}

// why not write an ad-hoc parser for this...
#[derive(Debug, PartialEq)]
enum Height { Cm, In }

#[derive(Debug)]
enum EyeColour { Amb, Blu, Brn, Gry, Grn, Hzl, Oth }

#[derive(Debug)]
struct TypedPassport {
  birth_year:       Option<i64>,
  issue_year:       Option<i64>,
  expiration_year:  Option<i64>,
  height:           Option<(i64, Height)>,
  hair_colour:      Option<u32>,
  eye_colour:       Option<EyeColour>,
  pid:              Option<String>,
  cid:              Option<String>,
}

fn parse_typed_passport(s: &str) -> Result<TypedPassport, String> {
  let mut passport = TypedPassport {
    birth_year:       None,
    issue_year:       None,
    expiration_year:  None,
    height:           None,
    hair_colour:      None,
    eye_colour:       None,
    pid:              None,
    cid:              None,
  };

  let parse_num = |it: &mut Peekable<Chars>| {
    let mut s = String::new();

    while let Some(&nc) = it.peek() {
      if !nc.is_numeric() {
        break;
      }
      s.push(it.next().unwrap());
    }

    s.parse::<i64>()
  };

  let parse_str = |it: &mut Peekable<Chars>| {
    let mut s = String::new();

    while let Some(&nc) = it.peek() {
      if nc == ' ' || nc == '\n' {
        break;
      }
      s.push(it.next().unwrap());
    }

    s
  };

  let mut it = s.chars().peekable();

  while let Some(&c) = it.peek() {
    match c {
      'a'..='z' => {
        let key: String = it.by_ref().take_while(|c| c.is_alphabetic()).collect();
        match key.as_str() {
          "byr" => passport.birth_year = parse_num(&mut it).ok(),
          "iyr" => passport.issue_year = parse_num(&mut it).ok(),
          "eyr" => passport.expiration_year = parse_num(&mut it).ok(),
          "cid" => passport.cid = Some(parse_str(&mut it)),
          "pid" => passport.pid = Some(parse_str(&mut it)),
          "hgt" => {
            match parse_num(&mut it) {
              Ok(height) => {
                let measure: String = it.by_ref().take(2).collect();
                passport.height = Some((height, match measure.as_str() {
                  "cm" => Height::Cm,
                  "in" => Height::In,
                  _ => return Err(format!("couldn't parse height measure")),
                }));
              },
              Err(_) => return Err(format!("couldn't parse height")),
            }
          }
          "hcl" => {
            if it.peek().is_some() && *it.peek().unwrap() == '#' {
              it.by_ref().next();
              let c: String = parse_str(&mut it);
              if c.len() == 6 {
                let num = u32::from_str_radix(c.as_str(), 16);
                passport.hair_colour = num.ok();
              }
            }
          }
          "ecl" => {
            passport.eye_colour = match parse_str(&mut it).as_str() {
              "amb" => Some(EyeColour::Amb),
              "blu" => Some(EyeColour::Blu),
              "brn" => Some(EyeColour::Brn),
              "gry" => Some(EyeColour::Gry),
              "grn" => Some(EyeColour::Grn),
              "hzl" => Some(EyeColour::Hzl),
              "oth" => Some(EyeColour::Oth),
              _ => return Err(format!("couldn't parse hair colour")),
            }
          }
          _ => ()
        }

        while let Some(&nc) = it.peek() {
          if nc == ' ' || nc == '\n' {
            it.next();
            break;
          }
          it.next();
        }
      }
      _ => {
        dbg!(passport);
        return Err(format!("expected a..z, but got {0}", c));
      }
    }
  }

  Ok(passport)
}

pub fn part_two(s: &str) -> i64 {
  let split = s.split("\n\n");
  let mut passports: Vec<TypedPassport> = Vec::new();

  for passport in split {
    match parse_typed_passport(passport) {
      Ok(p) => {
        if p.birth_year.is_none()
        || p.issue_year.is_none()
        || p.expiration_year.is_none()
        || p.height.is_none()
        || p.eye_colour.is_none()
        || p.hair_colour.is_none()
        || p.pid.is_none() {
          continue;
        }

        let byr = p.birth_year.unwrap();
        if byr < 1920 || byr > 2002 {
          continue;
        }

        let iyr = p.issue_year.unwrap();
        if iyr < 2010 || iyr > 2020 {
          continue;
        }

        let eyr = p.expiration_year.unwrap();
        if eyr < 2020 || eyr > 2030 {
          continue;
        }

        let hgt = p.height.as_ref().unwrap();
        if (hgt.1 == Height::Cm && (hgt.0 < 150 || hgt.0 > 193))
        || (hgt.1 == Height::In && (hgt.0 < 59 || hgt.0 > 76)) {
          continue;
        }

        let pid = p.pid.as_ref().unwrap();
        if pid.len() != 9  {
          continue;
        }

        passports.push(p);
      },
      Err(_) => (),
    }
  }

  passports.len() as i64
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"), 2);
  }

  #[test]
  fn test_part_one_bad_data() {
    assert_eq!(part_one("eyr:2033
hgt:170cm ecl:brn pid:#299cf2 hcl:#602927 byr:2004 iyr:2023"), 1);
  }

  #[test]
  fn test_part_two() {
    // invalid
    assert_eq!(part_two("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"), 0);

    // valid
    assert_eq!(part_two("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), 4);
  }
}