use std::collections::HashMap;
use regex::Regex;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(?P<number>[0-9]+) (?P<color>.+?) bag").unwrap();
    for line in input.lines() {
        let mut iter = line.split(" bags contain ");
        let color = iter.next().unwrap().to_string();
        let mut contains: Vec<(usize, String)> = Vec::new();
        for captures in re.captures_iter(line) {
            contains.push(((&captures["number"]).parse().unwrap(), (&captures["color"]).to_string()));
        }
        map.insert(color, contains);
    }
    map
}

#[aoc(day7, part1, unoptimized)]
pub fn solve_part1_unoptimized(map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut count = 0;
    let mut hold_map: HashMap<String, bool> = HashMap::new();
    for (color, _) in map.iter() {
        if can_hold(map, &mut hold_map, color) {
            count += 1;
        }
    }
    count
}

fn can_hold(map: &HashMap<String, Vec<(usize, String)>>, hold_map: &mut HashMap<String, bool>, current_color: &str) -> bool {
    if let Some(&can_hold) = hold_map.get(current_color) {
        return can_hold;
    };
    let vec = map.get(current_color).unwrap();
    for (_, color) in vec.iter() {
        if color == "shiny gold" || can_hold(map, hold_map, color) {
            hold_map.insert(current_color.to_string(), true);
            return true;
        }
    }
    hold_map.insert(current_color.to_string(), false);
    return false;
}

#[aoc(day7, part2, unoptimized)]
pub fn solve_part2_unoptimized(map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    sum_bag("shiny gold", map)
}

fn sum_bag(current_color: &str, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut sum = 0;
    let vec = map.get(current_color).unwrap();
    for (count, color) in vec.iter() {
        sum += count + count * sum_bag(color, map);
    }
    sum
}