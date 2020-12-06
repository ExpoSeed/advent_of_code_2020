#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .fold(0, |count, group| {
            let mut answered = [false; 26];
            let mut questions_count = 0;
            for letter in group.chars() {
                if letter == '\n' {
                    continue;
                }
                let index = (letter as usize) - ('a' as usize);
                if !answered[index] {
                    answered[index] = true;
                    questions_count += 1;
                }
            };
            count + questions_count
        })
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .fold(0, |count, group| {
            let mut person_count = 0;
            let mut answered_counts: [usize; 26] = [0; 26];
            for letter in group.chars() {
                if letter == '\n' {
                    person_count += 1;
                    continue;
                }
                let index = (letter as usize) - ('a' as usize);
                answered_counts[index] += 1;
            }
            count + answered_counts.iter().fold(0, |answered_count, &element| 
                if element > person_count {
                    answered_count + 1
                } else {
                    answered_count
                }
            )
        })
}