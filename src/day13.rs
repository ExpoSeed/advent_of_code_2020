#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> usize {
    struct Bus {id: usize, wait: usize};
    let mut lines = input.lines();
    let departure: usize = lines.next().unwrap().parse().unwrap();
    let bus: Bus = lines.next().unwrap()
        .split(',')
        .fold(None, |prev: Option<Bus>, id| {
            let id = match id.parse() {
                Ok(num) => num,
                _ => return prev, 
            };
            let wait = id - departure % id;
            match prev {
                Some(prev) => if wait < prev.wait {
                    Some(Bus{id, wait})
                } else {
                    Some(prev)
                },
                None => Some(Bus{id, wait})
            }
        })
        .unwrap();
    bus.id * bus.wait
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> usize {
    struct Bus {index: usize, id: usize};
    let mut lines = input.lines();
    let _ = lines.next();
    let mut buses = Vec::new();
    for (index, id) in lines.next().unwrap().split(",").enumerate() {
        if let Ok(id) = id.parse::<usize>() {
            buses.push(Bus{index, id});
        } 
    };
    
    let mut solution = 0;
    let mut lcd = 1;
    for bus in buses.iter() {
        while (solution + bus.index) % bus.id != 0 {
            solution += lcd;
        }
        lcd *= bus.id;
    };
    solution
}