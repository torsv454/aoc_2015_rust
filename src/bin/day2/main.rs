extern crate aoc_2015_rust;
use aoc_2015_rust::*;

use std::cmp::max;
use std::cmp::min;
use std::str::Split;

struct Box {
    width: u64,
    height: u64,
    depth: u64,
}

fn next_int(parts: &mut Split<&str>) -> u64 {
    parts
        .next()
        .expect("Expected width")
        .parse()
        .expect("number")
}

fn make_box(s: &str) -> Box {
    let mut parts = s.split("x");
    let width = next_int(&mut parts);
    let height = next_int(&mut parts);
    let depth = next_int(&mut parts);
    Box {
        width,
        height,
        depth,
    }
}

fn required_paper(b: &Box) -> u64 {
    let side1 = b.width * b.height;
    let side2 = b.width * b.depth;
    let side3 = b.height * b.depth;
    let extra = min(side1, min(side2, side3));
    2 * (side1 + side2 + side3) + extra
}

fn required_ribbon(b: &Box) -> u64 {
    let bow = b.width * b.height * b.depth;
    let largest = max(b.width, max(b.height, b.depth));
    let wrapping = 2 * (b.width + b.height + b.depth - largest);
    wrapping + bow
}

fn boxes(input: &str) -> Vec<Box> {
    let lines = input.lines();
    lines.map(make_box).collect()
}

fn part1(input: &str) -> u64 {
    boxes(input).iter().map(required_paper).sum()
}

fn part2(input: &str) -> u64 {
    boxes(input).iter().map(required_ribbon).sum()
}

fn main() {
    let input = input(2);
    println!("AoC 2015 Day 2, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 2, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_samples() {
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn day2_part2_samples() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }

    #[test]
    fn day2_part1_correct_answer() {
        assert_eq!(part1(&input(2)), 1606483);
    }

    #[test]
    fn day2_part2_correct_answer() {
        assert_eq!(part2(&input(2)), 3842356);
    }

}
