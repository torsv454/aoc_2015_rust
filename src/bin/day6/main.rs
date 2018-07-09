extern crate aoc_2015_rust;
use aoc_2015_rust::*;

use std::process;
use std::str::SplitWhitespace;

struct Area {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

enum Cmd {
    Toggle(Area),
    On(Area),
    Off(Area),
}

const TOGGLE: Option<&str> = Some("toggle");
const TURN: Option<&str> = Some("turn");
const ON: Option<&str> = Some("on");
const OFF: Option<&str> = Some("off");

fn parse_pair(pair: &str) -> (usize, usize) {
    let mut parts = pair.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn parse_area(iter: &mut SplitWhitespace) -> Area {
    let (x1, y1) = parse_pair(iter.next().unwrap());
    iter.next();
    let (x2, y2) = parse_pair(iter.next().unwrap());
    Area { x1, y1, x2, y2 }
}

fn parse_cmd(line: &str) -> Cmd {
    let mut parts = line.split_whitespace();
    let first = parts.next();
    if TOGGLE == first {
        return Cmd::Toggle(parse_area(&mut parts));
    } else if TURN == first {
        let second = parts.next();
        if ON == second {
            return Cmd::On(parse_area(&mut parts));
        } else if OFF == second {
            return Cmd::Off(parse_area(&mut parts));
        }
    }
    println!("Unknown line {}", line);
    process::abort();
}

fn parse(string: &str) -> Vec<Cmd> {
    let cmds: Vec<Cmd> = string.lines().map(|x| parse_cmd(x)).collect();
    cmds
}

fn toggle(i: u64) -> u64 {
    if i == 1 {
        0
    } else {
        1
    }
}

fn on(_: u64) -> u64 {
    1
}

fn off(_: u64) -> u64 {
    0
}

fn inc(i: u64) -> u64 {
    i + 1
}

fn dec(i: u64) -> u64 {
    if i > 0 {
        i - 1
    } else {
        0
    }
}

fn inc2(i: u64) -> u64 {
    i + 2
}

fn change(transform: fn(u64) -> u64, area: &Area, lights: &mut [u64]) {
    for x in area.x1..=area.x2 {
        for y in area.y1..=area.y2 {
            let i = x * 1000 + y;
            lights[i] = transform(lights[i]);
        }
    }
}

fn perform(cmd: &Cmd, lights: &mut [u64]) {
    match cmd {
        Cmd::Toggle(area) => change(toggle, area, lights),
        Cmd::On(area) => change(on, area, lights),
        Cmd::Off(area) => change(off, area, lights),
    }
}

fn perform2(cmd: &Cmd, lights: &mut [u64]) {
    match cmd {
        Cmd::Toggle(area) => change(inc2, area, lights),
        Cmd::On(area) => change(inc, area, lights),
        Cmd::Off(area) => change(dec, area, lights),
    }
}

fn part1(input: &str) -> usize {
    let mut lights: [u64; 1_000_000] = [0; 1_000_000];
    let cmds = parse(input);

    for c in &cmds {
        perform(&c, &mut lights);
    }

    lights.iter().filter(|x| **x == 1).count()
}

fn part2(input: &str) -> u64 {
    let mut lights: [u64; 1_000_000] = [0; 1_000_000];
    let cmds = parse(input);
    for c in &cmds {
        perform2(&c, &mut lights);
    }

    lights.iter().sum()
}

fn main() {
    let input = input(6);
    println!("AoC 2015 Day 6, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 6, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    // WTF, why does this overflow stack as a test but not when run?
    //#[test]
    //fn day6_part1_samples() {
    //    assert_eq!(part1("turn on 0,0 through 999,999"), 1_000_000);
    //    assert_eq!(part1("toggle 0,0 through 999,0"), 1_000);
    //    assert_eq!(part1("turn off 499,499 through 500,500"), 0);
    //}

    //#[test]
    //fn day6_part1_correct_answer() {
    //    assert_eq!(part1(&input(6)), 377891);
    // }

    //#[test]
    //fn day6_part2_samples() {
    //    //assert_eq!(part2("turn on 0,0 through 0,0"), 1);
    //    assert_eq!(part2("toggle 0,0 through 999,0"), 1_000);
    //    assert_eq!(part2("toggle 0,0 through 999,999"), 2_000_000);
    //}

    //#[test]
    //fn day6_part2_correct_answer() {
    //    assert_eq!(part2(&input(6)), 14110788);
    //}
}
