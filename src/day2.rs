pub struct Password {
    num1: u32,
    num2: u32,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split(" ").collect();
            let bounds: Vec<&str> = v[0].split("-").collect();
            Password{
                num1: bounds[0].parse().unwrap(),
                num2: bounds[1].parse().unwrap(),
                letter: v[1].chars().next().unwrap(),
                password: String::from(v[2])
            }            
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(passwords: &[Password]) -> u32 {
    passwords
        .iter()
        .fold(0, |count, entry| {
            let letter_count = entry.password.matches(entry.letter).count() as u32;
            count + if letter_count >= entry.num1 && letter_count <= entry.num2 {
                1
            } else {
                0
            }
        })
}

#[aoc(day2, part2)]
pub fn solve_part2(passwords: &[Password]) -> u32 {
    passwords
        .iter()
        .fold(0, |count, entry| {
            let bytes = entry.password.as_bytes();
            count + if (bytes[entry.num1 as usize - 1] as char == entry.letter) ^ 
                       (bytes[entry.num2 as usize - 1] as char == entry.letter) {
                1
            } else {
                0
            }
        })
}