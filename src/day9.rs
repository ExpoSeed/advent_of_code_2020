#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut v: Vec<usize> = Vec::new();
    let mut lines = input.lines();
    // preamble
    for _ in 0..25 {
        v.push(lines
            .next().expect("unexpected end of file")
            .parse().expect("error parsing")
        );
    };

    // lines
    //     .map(|line| {
    //         let num = line.parse::<usize>().expect("error parsing");
    //         v.push(num);
    //         num
    //     })
    //     .find(|&num| !is_valid(&v[v.len() - 26..v.len() - 1], num))
    //     .expect("no invalid number")
    for line in lines {
        let num = line.parse::<usize>().expect("error parsing");
        v.push(num);
        if !is_valid(&v[v.len() - 26..v.len() - 1], num) {
            return num;
        };
    };
    panic!("no invalid number")
}

fn is_valid(nums: &[usize], tgt: usize) -> bool {
    for (index1, element1) in (&nums[0..nums.len()-1]).iter().enumerate() {
        for element2 in (&nums[index1..]).iter() {
            if element1 + element2 == tgt {
                return true;
            }
        }
    };
    return false;
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let invalid_number = solve_part1(input);
    let mut v: Vec<usize> = Vec::new();
    let mut start_index = 0;
    let mut lines = input.lines();
    loop {
        let mut min = *match v.get(start_index) {
            Some(n) => n,
            None => {
                v.push(lines
                    .next().expect("unexpected end of file")
                    .parse().expect("error parsing"));
                continue;
            },
        };
        let mut max = min;
        let mut end_index = start_index + 1;
        let mut sum = min;
        while sum < invalid_number {
            let val = *match v.get(end_index) {
                Some(n) => n,
                None => {
                    v.push(lines
                        .next().expect("unexpected end of file")
                        .parse().expect("error parsing"));
                    continue;
                },
            };
            sum += val;
            if val > max { max = val; };
            if val < min { min = val; };
            if sum == invalid_number {
                return min + max;
            }
            end_index += 1;
        } 
        start_index += 1;
    }
}