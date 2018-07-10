extern crate aoc_2015_rust;
use aoc_2015_rust::*;

#[derive(Debug)]
enum Instr {
    Hlf { register: usize },
    Tpl { register: usize },
    Inc { register: usize },
    Jmp { offset: i32 },
    Jie { register: usize, offset: i32 },
    Jio { register: usize, offset: i32 },
}

#[derive(Debug)]
struct Program(Vec<Instr>);

#[derive(Debug)]
struct Machine<'a> {
    registers: Vec<u64>,
    program: &'a Program,
    pc: i32,
}

fn str_to_register(text: &str) -> usize {
    match text {
        "a" | "a," => 0,
        "b" | "b," => 1,
        _ => panic!("Invalid register"),
    }
}

fn str_to_offset(text: &str) -> i32 {
    let sign = &text[0..1];
    let num: i32 = text[1..].parse().unwrap();
    if sign == "-" {
        -num
    } else {
        num
    }
}

fn parse(input: &str) -> Program {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "hlf" => instructions.push(Instr::Hlf {
                register: str_to_register(parts[1]),
            }),
            "tpl" => instructions.push(Instr::Tpl {
                register: str_to_register(parts[1]),
            }),
            "inc" => instructions.push(Instr::Inc {
                register: str_to_register(parts[1]),
            }),
            "jmp" => instructions.push(Instr::Jmp {
                offset: str_to_offset(parts[1]),
            }),
            "jie" => instructions.push(Instr::Jie {
                register: str_to_register(parts[1]),
                offset: str_to_offset(parts[2]),
            }),
            "jio" => instructions.push(Instr::Jio {
                register: str_to_register(parts[1]),
                offset: str_to_offset(parts[2]),
            }),
            _ => panic!("Unknown instruction."),
        }
    }

    Program(instructions)
}

fn run(machine: &mut Machine) {
    while machine.pc >= 0 && machine.pc < machine.program.0.len() as i32 {
        match machine.program.0[machine.pc as usize] {
            Instr::Hlf { register } => {
                machine.registers[register] /= 2;
                machine.pc += 1;
            }
            Instr::Tpl { register } => {
                machine.registers[register] *= 3;
                machine.pc += 1;
            }
            Instr::Inc { register } => {
                machine.registers[register] += 1;
                machine.pc += 1;
            }
            Instr::Jmp { offset } => {
                machine.pc += offset;
            }
            Instr::Jie { register, offset } => {
                if machine.registers[register] % 2 == 0 {
                    machine.pc += offset;
                } else {
                    machine.pc += 1;
                }
            }
            Instr::Jio { register, offset } => {
                if machine.registers[register] == 1 {
                    machine.pc += offset;
                } else {
                    machine.pc += 1;
                }
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    let program = parse(input);
    let mut machine = Machine {
        registers: vec![0; 2],
        program: &program,
        pc: 0,
    };
    run(&mut machine);
    machine.registers[1]
}

fn part2(input: &str) -> u64 {
    let program = parse(input);
    let mut machine = Machine {
        registers: vec![1, 0],
        program: &program,
        pc: 0,
    };
    run(&mut machine);
    machine.registers[1]
}

fn main() {
    let input = input(23);
    println!("AoC 2015 Day 23, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 23, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part1_correct_answer() {
        assert_eq!(part1(&input(23)), 307);
    }

    #[test]
    fn day23_part2_correct_answer() {
        assert_eq!(part2(&input(23)), 160);
    }

}
