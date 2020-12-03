#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    for (index1, element1) in (&input[0..input.len()-1]).iter().enumerate() {
        for element2 in (&input[index1 as usize..]).iter() {
            if element1 + element2 == 2020 {
                return element1 * element2;
            }
        }
    }
    unreachable!()
}
