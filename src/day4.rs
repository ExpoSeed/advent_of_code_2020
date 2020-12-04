use std::collections::HashSet;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().copied().collect();
    input
        .split("\n\n")
        .fold(0, |count, passport| {
            let fields: HashSet<&str> = passport
                .split_whitespace()
                .map(|entry| entry.split(":").next().unwrap())
                .collect();
            if required_fields.is_subset(&fields) {
                count + 1
            } else {
                count
            }
        })
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().copied().collect();
    input
        .split("\n\n")
        .fold(0, |count, passport| {
            let mut present_fields: HashSet<&str> = HashSet::new();
            if passport
                .split_whitespace()
                .all(|entry| present_fields.insert(entry.split(":").next().unwrap()) && is_valid_entry(entry)) &&
                required_fields.is_subset(&present_fields) {
                count + 1
            } else {
                count
            }
        })
}

fn is_valid_entry(entry: &str) -> bool {
    let mut iter = entry.split(":");
    let first = iter.next().unwrap();
    if first == "cid" {
        return true;
    }
    let value = if let Some(x) = iter.next() {
        x
    } else {
        return false;
    };

    match first {
        "byr" => matches!(value.parse::<usize>(), Ok(number) if (1920..=2002).contains(&number)),
        "iyr" => matches!(value.parse::<usize>(), Ok(number) if (2010..=2020).contains(&number)),
        "eyr" => matches!(value.parse::<usize>(), Ok(number) if (2020..=2030).contains(&number)),
        "hgt" => {
            let (height, unit) = value.split_at(value.len() - 2);
            let range = match unit {
                "cm" => 150..=193,
                "in" => 59..=76,
                _ => return false
            };
            matches!(height.parse::<usize>(), Ok(number) if range.contains(&number))
        }
        "hcl" => value.starts_with('#') && value[1..].chars().all(|byte| byte.is_digit(16)),
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => value.len() == 9 && value.parse::<usize>().is_ok(),
        _ => panic!("bad field")
    }
}

// is 70-80 us faster for some reason
fn is_valid_entry_old(entry: &str) -> bool {
    // println!("{}", entry.split(":").nth(1).unwrap());
    let mut iter = entry.split(":");
    match iter.next().unwrap() {
        "byr" => match iter.next() {
            Some(value) => match value.parse::<usize>() {
                Ok(number) => number >= 1920 && number <= 2002,
                Err(_) => false,
            }
            None => false
        },
        "iyr" => match iter.next() {
            Some(value) => match value.parse::<usize>() {
                Ok(number) => number >= 2010 && number <= 2020,
                Err(_) => false,
            }
            None => false
        },
        "eyr" => match iter.next() {
            Some(value) => match value.parse::<usize>() {
                Ok(number) => number >= 2020 && number <= 2030,
                Err(_) => false,
            }
            None => false
        },
        "hgt" => match iter.next() {
            Some(value) => match &value[value.len()-2..] {
                "cm" => match (&value[..value.len()-2]).parse::<usize>() {
                    Ok(number) => number >= 150 && number <= 193,
                    Err(_) => false,
                },
                "in" => match (&value[..value.len()-2]).parse::<usize>() {
                    Ok(number) => number >= 59 && number <= 76,
                    Err(_) => false,
                },
                _ => false,
            },
            None => false,

        },
        "hcl" => match iter.next() {
            Some(value) => value.starts_with('#') && value[1..].chars().all(|byte| byte.is_digit(16)),
            None => false,
        },
        "ecl" => match iter.next() {
            Some("amb") | Some("blu") | Some("brn") | Some("gry") | Some("grn") | Some("hzl") | Some("oth") => true,
            _ => false,
        },
        "pid" => match iter.next() {
            Some(value) => value.len() == 9 && value.parse::<usize>().is_ok(),
            None => false,
        },
        "cid" => true,
        _ => panic!("bad field"),
    }
}