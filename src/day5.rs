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

fn get_seat_id(line: &str) -> usize {
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