#[aoc_generator(day10, part1)]
pub fn generator_part1(input: &str) -> Vec<usize> {
    let mut v: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap_or_else(|err| panic!("Error parsing: {:?}", err)))
        .collect();
    v.sort_unstable();
    v
}

#[aoc(day10, part1)]
pub fn solve_part1(adapters: &[usize]) -> usize {
    let (mut diffs_1, mut diffs_3) = (0, 1);
    let mut current = 0;
    for &adapter in adapters {
        match adapter - current {
            1 => {diffs_1 += 1; current = adapter;},
            2 => current = adapter,
            3 => {diffs_3 += 1; current = adapter}
            _ => (),
        }
    }
    diffs_1 * diffs_3
}



use std::collections::HashSet;
#[aoc_generator(day10, part2)]
pub fn generator_part2(input: &str) -> HashSet<isize> {
    input
        .lines()
        .map(|line| line.parse().unwrap_or_else(|err| panic!("Error parsing: {:?}", err)))
        .collect()
}

use std::collections::HashMap;
// this was too difficult for me :(
#[aoc(day10, part2)]
pub fn solve_part2(adapters: &HashSet<isize>) -> isize{
    count_arrangements(adapters, *adapters.iter().max().unwrap(), &mut HashMap::new())
}

fn count_arrangements(adapters: &HashSet<isize>, value: isize, memo: &mut HashMap<isize, isize>) -> isize {
    if let Some(&ret) = memo.get(&value) {
        return ret;
    };
    if value == 0 {
        memo.insert(value, 1);
        return 1;
    }
    if value < 0 || !adapters.contains(&value) {
        memo.insert(value, 0);
        return 0;
    };
    let result = count_arrangements(adapters, value - 1, memo) +
                 count_arrangements(adapters, value - 2, memo) +
                 count_arrangements(adapters, value - 3, memo);
    memo.insert(value, result);
    return result;
}