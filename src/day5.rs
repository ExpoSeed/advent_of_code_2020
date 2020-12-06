#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |current_max, line| {
            let seat_id = get_seat_id(line);
            if seat_id > current_max {
                seat_id
            } else {
                current_max
            }
        })
}

// with the binary trick
#[aoc(day5, part1, optimized)]
pub fn solve_part1_optimized(input: &str) -> usize {
    input
        .lines()
        .fold(0, |current_max, line| {
            let mut seat_id = 0;
            for (index, letter) in line.chars().enumerate() {
                seat_id |= match letter {
                    'B' | 'R' => 1,
                    'F' | 'L' => 0,
                    invalid => panic!("invalid input: {}", invalid),
                } << (10 - index);
            }
            if seat_id > current_max {
                seat_id
            } else {
                current_max
            }
        })
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let ids: Vec<usize> = input
        .lines()
        .map(|line| get_seat_id(line))
        .collect();

    for (index1, element1) in (&ids[0..ids.len()-1]).iter().enumerate() {
        for element2 in (&ids[index1..]).iter() {
            if (element1 - element2 == 2) || (element2 - element1 == 2) {
                let avg = (element1 + element2)/2;
                if !ids.contains(&avg) {
                    return avg;
                }
            }
        }
    }
    unreachable!()
}

// SBird did a linear search for the missing seat
// Knarkzel interpreted all as one number instead of two
// I thought of boolean array to avoid a heap allocation
// possibly 
#[aoc(day5, part2, optimized)]
pub fn solve_part2_optimized(input: &str) -> usize {
    let mut present_seats = [false; 1024];
    for line in input.lines() {
        let mut num = 0;
        for (index, letter) in line.chars().enumerate() {
            num |= match letter {
                'B' | 'R' => 1,
                'F' | 'L' => 0,
                invalid => panic!("invalid input: {}", invalid),
            } << (9 - index);
        }
        present_seats[num] = true;
    };

    for seat in 1..1024 {
        if present_seats[seat - 1] && !present_seats[seat] && present_seats[seat + 1] {
            return seat;
        }
    };
    panic!("seat not found")
}

// much more clever! thank you ShantyTown!
// the positions are just binary numbers! 
fn get_seat_id(line: &str) -> usize {
    let mut iter = line.chars();
    let row = {
        let mut num = 0;
        for i in (0..7).rev() {
            num |= convert_to_bit(iter.next()) << i;
        }
        num
    };
    let col = {
        let mut num = 0;
        for i in (0..3).rev() {
            num |= convert_to_bit(iter.next()) << i;
        }
        num
    };
    row * 8 + col
}

fn convert_to_bit(letter: Option<char>) -> usize {
    match letter {
        Some('B') | Some('R') => 1,
        Some('F') | Some('L') => 0,
        _ => panic!("bad input"),
    }
}

// my original solution
#[allow(dead_code)]
fn get_seat_id_bounds(line: &str) -> usize {
    let mut iter = line.chars();
    let row = {
        let mut lower_bound = 0;
        let mut upper_bound = 128;
        while lower_bound != upper_bound - 1 {
            match iter.next() {
                Some('B') => lower_bound = (upper_bound + lower_bound)/2,
                Some('F') => upper_bound = (upper_bound + lower_bound)/2,
                _ => panic!("bad row input"),
            };
        };
        lower_bound
    };
    let col = {
        let mut lower_bound = 0;
        let mut upper_bound = 8;
        while lower_bound != upper_bound - 1 {
            match iter.next() {
                Some('R') => lower_bound = (upper_bound + lower_bound)/2,
                Some('L') => upper_bound = (upper_bound + lower_bound)/2,
                _ => panic!("bad col input"),
            };
        };
        lower_bound
    };
    (row * 8) + col
}