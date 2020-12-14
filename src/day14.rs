use std::collections::HashMap;
use regex::Regex;

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let re_mask = Regex::new(r"mask = (?P<mask>[10X]+)").unwrap();
    let re_mem = Regex::new(r"mem\[(?P<address>[0-9]+)\] = (?P<value>[0-9]+)").unwrap();
    let mut mask: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        if re_mask.is_match(line) {
            mask = HashMap::new();
            for (i, bit) in re_mask.captures(line).unwrap().name("mask").unwrap().as_str().chars().rev().enumerate() {
                match bit {
                    '1' => mask.insert(i, 1),
                    '0' => mask.insert(i, 0),
                    'X' => None,
                    e => panic!("invalid bit: {}", e),
                };
            };
        } else { // assume re_mem matches
            let captures = re_mem.captures(line).unwrap();
            let mut value = captures.name("value").unwrap().as_str().parse().unwrap();
            for (i, bit) in mask.iter() {
                value = (value & !(1 << i)) | (bit << i);
            }
            let address = captures.name("address").unwrap().as_str().parse().unwrap();
            memory.insert(address, value);
        }
    }
    memory.iter().fold(0, |count, (_, val)| count + val)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let re_mask = Regex::new(r"mask = (?P<mask>[10X]+)").unwrap();
    let re_mem = Regex::new(r"mem\[(?P<address>[0-9]+)\] = (?P<value>[0-9]+)").unwrap();
    let mut mask: Vec<usize> = Vec::new();
    for line in input.lines() {
        if re_mask.is_match(line) {
            mask = Vec::new();
            for bit in re_mask.captures(line).unwrap().name("mask").unwrap().as_str().chars().rev() {
                match bit {
                    '1' => mask.push(1),
                    '0' => mask.push(0),
                    'X' => mask.push(2),
                    e => panic!("invalid bit: {}", e),
                };
            };
        } else { // assume re_mem matches
            let captures = re_mem.captures(line).unwrap();
            let value = captures.name("value").unwrap().as_str().parse().unwrap();
            let address = captures.name("address").unwrap().as_str().parse().unwrap();
            write(address, value, 0, &mut memory, &mask);
        }
    }
    memory.iter().fold(0, |count, (_, val)| count + val)
}

fn write(address: usize, value: usize, current_index: usize, memory: &mut HashMap<usize, usize>, mask: &[usize]) {
    if current_index >= mask.len() {
        memory.insert(address, value);
        return;
    }
    match mask[current_index] {
        0 => write(address, value, current_index + 1, memory, mask),
        1 => {
            let address = (address & !(1 << current_index)) | (1 << current_index);
            write(address, value, current_index + 1, memory, mask);
        },
        2 => {
            let address = address & !(1 << current_index);
            write(address, value, current_index + 1, memory, mask);
            let address = (address & !(1 << current_index)) | (1 << current_index);
            write(address, value, current_index + 1, memory, mask);
        },
        _ => panic!("wat"),
    }
}