#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut old_seats: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut temp = Vec::new();
        for letter in line.chars() {
            temp.push(letter);
        }
        old_seats.push(temp);
    };

    loop {
        let mut changed = false;
        let mut new_seats: Vec<Vec<char>> = Vec::new();
        for i in 0..old_seats.len() {
            let mut temp = Vec::new();
            for j in 0..old_seats[i].len() {
                let current_seat = old_seats[i][j];
                if current_seat == '.' {
                    temp.push('.');
                } else {
                    let mut count = 0;
                    let i = i - 1;
                    let j = j - 1;
                    for row_offset in 0..3 {
                        for col_offset in 0..3 {
                            if row_offset == 1 && col_offset == 1 {
                                continue;
                            }
                            let adjacent_seat = old_seats.get(i + row_offset).and_then(|row| row.get(j + col_offset));
                            if let Some('#') = adjacent_seat {
                                count += 1;
                            }
                        }
                    }
                    temp.push(match current_seat {
                        'L' => if count == 0 {changed = true; '#'} else {'L'},
                        '#' => if count >= 4 {changed = true; 'L'} else {'#'},
                        c => panic!("invalid character: {}", c),
                    });
                }
            }
            new_seats.push(temp);
        }
        if !changed {
            break;
        } else {
            old_seats = new_seats;
        }
    }

    old_seats
        .iter()
        .flatten()
        .fold(0, |count, &seat| count + if seat == '#' {1} else {0})
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut old_seats: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut temp = Vec::new();
        for letter in line.chars() {
            temp.push(letter);
        }
        old_seats.push(temp);
    };


    loop {
        let mut changed = false;
        let mut new_seats: Vec<Vec<char>> = Vec::new();
        for i in 0..old_seats.len() {
            let mut temp = Vec::new();
            for j in 0..old_seats[i].len() {
                let current_seat = old_seats[i][j];
                if current_seat == '.' {
                    temp.push('.');
                } else {
                    let mut count = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let mut i = i as isize + dx;
                            let mut j = j as isize + dy;
                            while let Some(s) = old_seats.get(i as usize).and_then(|row| row.get(j as usize)) {
                                match s {
                                    '#' => {count += 1; break},
                                    'L' => break,
                                    _ => ()
                                }
                                i = i as isize + dx;
                                j = j as isize + dy;
                            }
                        }
                    }

                    temp.push(match current_seat {
                        'L' => if count == 0 {changed = true; '#'} else {'L'},
                        '#' => if count >= 5 {changed = true; 'L'} else {'#'},
                        c => panic!("invalid character: {}", c),
                    });
                }
            }
            new_seats.push(temp);
        }
        if !changed {
            break;
        } else {
            old_seats = new_seats;
        }
    }

    old_seats
        .iter()
        .flatten()
        .fold(0, |count, &seat| count + if seat == '#' {1} else {0})
}