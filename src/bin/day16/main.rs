extern crate aoc_2015_rust;
use aoc_2015_rust::*;

use std::collections::HashMap;

fn matches(values: &HashMap<String, i32>, sue: &HashMap<String, i32>) -> bool {
    for (key, value) in values {
        if let Some(actual) = sue.get(key) {
            if actual != value {
                return false;
            }
        }
    }
    true
}

fn matches2(values: &HashMap<String, i32>, sue: &HashMap<String, i32>) -> bool {
    for (key, value) in values {
        if let Some(actual) = sue.get(key) {
            match &key[..] {
                "cats:" | "trees:" => if value >= actual {
                    return false;
                },
                "pomeranians:" | "goldfish:" => if value <= actual {
                    return false;
                },
                _ => if value != actual {
                    return false;
                },
            }
        }
    }
    true
}

fn int_of(text: &str) -> i32 {
    let len = text.len();
    let last = text.chars().last().unwrap();
    if last == ':' || last == ',' {
        &text[..len - 1]
    } else {
        &text[..]
    }.parse()
        .unwrap()
}

fn parse_line(line: &str) -> HashMap<String, i32> {
    let parts: Vec<&str> = line.split(' ').collect();

    let mut m = HashMap::new();

    let mut i = 0;
    while i < parts.len() {
        m.insert(parts[i].into(), int_of(parts[i + 1]));
        i += 2;
    }

    m
}

fn part1(input: &str) -> i32 {
    let sample = parse_line(r"Sample 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1");
    let sues: Vec<HashMap<String, i32>> = input.lines().map(parse_line).collect();

    *sues.iter()
        .find(|s| matches(&sample, s))
        .expect("No sue found :(")
        .get("Sue")
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let sample = parse_line(r"Sample 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1");
    let sues: Vec<HashMap<String, i32>> = input.lines().map(parse_line).collect();

    *sues.iter()
        .find(|s| matches2(&sample, s))
        .expect("No sue found :(")
        .get("Sue")
        .unwrap()
}

fn main() {
    let input = input(16);
    println!("AoC 2015 Day 16, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 16, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_correct_answer() {
        assert_eq!(part1(&input(16)), 373);
    }

    #[test]
    fn day16_part2_correct_answer() {
        assert_eq!(part2(&input(16)), 260);
    }

}
