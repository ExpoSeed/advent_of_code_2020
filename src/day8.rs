#[derive(Debug, Copy, Clone)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    argument: isize,
    executed: bool,
}

fn convert_to_instruction(s: &str) -> Instruction {
    let mut iter = s.split(" ");
    let opcode = match iter.next() {
        Some("acc") => Opcode::Acc,
        Some("jmp") => Opcode::Jmp,
        Some("nop") => Opcode::Nop,
        _ => panic!("bad opcode"),
    };
    let argument = iter.next().unwrap().parse::<isize>().unwrap();
    Instruction{opcode, argument, executed: false}
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> isize {
    let mut asm: Vec<Instruction> = Vec::new();
    let mut ip: isize = 0; // instruction pointer
    let mut acc: isize = 0;
    let mut lines = input.lines();
    loop {
        let current = match asm.get_mut(ip as usize) {
            Some(instruction) => instruction,
            None => {
                asm.push(convert_to_instruction(lines.next().unwrap()));
                continue
            },
        };
        if current.executed { break; }
        match current.opcode {
            Opcode::Acc => {
                acc += current.argument;
                ip += 1;
            },
            Opcode::Jmp => {
                ip += current.argument;
            },
            Opcode::Nop => {
                ip += 1;
            }
        };
        current.executed = true;
    };
    acc
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> isize {
    let mut asm: Vec<Instruction> = Vec::new();
    let mut lines = input.lines();
    let mut index = 0;
    loop {
        // let mut instruction = convert_to_instruction(lines.next().unwrap());
        let mut instruction = match asm.get_mut(index) {
            Some(value) => value,
            None => {
                asm.push(convert_to_instruction(lines.next().unwrap()));
                continue;
            }
        };
        match instruction.opcode {
            Opcode::Jmp => instruction.opcode = Opcode::Nop,
            Opcode::Nop => instruction.opcode = Opcode::Jmp,
            _ => (),
        }
        if let Some(acc) = terminates(&mut asm, &mut lines) {
            return acc;
        }
        match asm[index].opcode {
            Opcode::Jmp => asm[index].opcode = Opcode::Nop,
            Opcode::Nop => asm[index].opcode = Opcode::Jmp,
            _ => (),
        };
        index += 1;
    }
}

fn terminates(asm: &mut Vec<Instruction>, lines: &mut std::str::Lines) -> Option<isize> {
    let mut ip: isize = 0; // instruction pointer
    let mut acc: isize = 0;
    loop {
        let current = match asm.get_mut(ip as usize) {
            Some(instruction) => instruction,
            None => {
                match lines.next() {
                    Some(line) => {
                        asm.push(convert_to_instruction(line));
                        continue;
                    },
                    None => {
                        reset_asm(asm);
                        return Some(acc);
                    }
                }
            },
        };
        if current.executed { break; }
        match current.opcode {
            Opcode::Acc => {
                acc += current.argument;
                ip += 1;
            },
            Opcode::Jmp => {
                ip += current.argument;
            },
            Opcode::Nop => {
                ip += 1;
            }
        };
        current.executed = true;
    };
    reset_asm(asm);
    return None;
}

fn reset_asm(asm: &mut Vec<Instruction>) {
    for instruction in asm.iter_mut() {
        instruction.executed = false;
    }
}