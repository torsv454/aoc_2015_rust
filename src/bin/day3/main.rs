extern crate aoc_2015_rust;
use aoc_2015_rust::*;

use std::iter::Iterator;

fn step((x, y): (i64, i64), dir: char) -> (i64, i64) {
    match dir {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => panic!("unknown direction"),
    }
}

fn walk(chars: &str, positions: &mut Vec<(i64, i64)>) {
    let mut curr: (i64, i64) = (0, 0);
    positions.push(curr);
    for c in chars.chars() {
        curr = step(curr, c);
        positions.push(curr);
    }
}

fn unique(positions: &mut Vec<(i64, i64)>) -> usize {
    positions.sort();
    positions.dedup();
    positions.len()
}

fn every_other(source: &str, offset: usize) -> String {
    source
        .chars()
        .enumerate()
        .filter(|&(i, _)| i % 2 == offset)
        .map(|(_, e)| e)
        .collect()
}

fn part1(input: &str) -> usize {
    let mut part1: Vec<(i64, i64)> = Vec::new();
    walk(input, &mut part1);
    unique(&mut part1)
}

fn part2(input: &str) -> usize {
    let mut part2: Vec<(i64, i64)> = Vec::new();
    let santas_steps = every_other(input, 0);
    let robo_steps = every_other(input, 1);
    walk(santas_steps.as_str(), &mut part2);
    walk(robo_steps.as_str(), &mut part2);
    unique(&mut part2)
}

fn main() {
    let input = input(3);
    println!("AoC 2015 Day 3, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 3, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_correct_answer() {
        assert_eq!(part1(&input(3)), 2572);
    }

    #[test]
    fn day3_part2_correct_answer() {
        assert_eq!(part2(&input(3)), 2631);
    }

}
