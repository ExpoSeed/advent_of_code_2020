#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut count: u32 = 0;
    let mut pos: usize = 3;
    let mut iter = input.lines();
    let size = iter.next().unwrap().chars().count();
    for line in iter {
        if line.chars().nth(pos).unwrap() == '#' {
            count += 1;
        }
        pos = (pos + 3) % size;
    }
    return count;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    // (rows right, columns down)
    let vectors: [(usize, usize);5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // (current row, current count)
    let mut states: [(usize, usize);5] = [(0,0);5];
    for (line_number, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().map(|byte| byte as char).collect();
        for (state, vector) in states.iter_mut().zip(vectors.iter()) {
            if line_number % vector.1 == 0 {
                if chars[state.0] == '#' {
                    state.1 += 1;
                }
                state.0 = (state.0 + vector.0) % chars.len();
            }
        }
    }
    return states.iter().fold(1, |product, state| product * state.1 as u32);
}