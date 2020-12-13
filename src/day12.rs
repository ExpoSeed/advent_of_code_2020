#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut facing: isize = 0;
    let mut directions: [isize; 4] = [0; 4];
    for line in input.lines() {
        let (action, value) = line.split_at(1);
        let value = value.parse::<isize>().unwrap();
        match action {
            "N" => directions[1] += value,
            "S" => directions[3] += value,
            "E" => directions[0] += value,
            "W" => directions[2] += value,
            "L" => facing = ((facing + value / 90) + 4) % 4,
            "R" => facing = ((facing - value / 90) + 4) % 4,
            "F" => directions[facing as usize] += value,
            l => panic!("invalid letter: {}", l),
        }
    }
    (directions[0] - directions[2]).abs() as usize +
    (directions[1] - directions[3]).abs() as usize
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut ship: [isize; 4] = [0; 4];
    let mut waypoint: [isize; 4] = [10, 1, 0, 0];
    for line in input.lines() {
        let (action, value) = line.split_at(1);
        let value = value.parse::<isize>().unwrap();
        match action {
            "N" => waypoint[1] += value,
            "S" => waypoint[3] += value,
            "E" => waypoint[0] += value,
            "W" => waypoint[2] += value,
            "R" => {
                for _ in 0..(value / 90) {
                    waypoint = [waypoint[1], waypoint[2], waypoint[3], waypoint[0]];
                }
            },
            "L" => {
                for _ in 0..(value / 90) {
                    waypoint = [waypoint[3], waypoint[0], waypoint[1], waypoint[2]];
                }
            },
            "F" => {
                for _ in 0..value {
                    ship[0] += waypoint[0];
                    ship[1] += waypoint[1];
                    ship[2] += waypoint[2];
                    ship[3] += waypoint[3];
                }
            }
            l => panic!("invalid letter: {}", l),
        }
    }
    (ship[0] - ship[2]).abs() as usize +
    (ship[1] - ship[3]).abs() as usize
}
