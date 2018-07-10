extern crate aoc_2015_rust;
use aoc_2015_rust::*;

extern crate regex;
extern crate serde_json;

use regex::Regex;
use serde_json::map::Map;
use serde_json::{Error, Value};

fn valid(props: &Map<String, Value>) -> bool {
    for p in props.values() {
        if let Value::String(v) = p {
            if "red" == v {
                return false;
            }
        }
    }
    true
}

fn part2(text: &str) -> Result<i64, Error> {
    let v: Value = serde_json::from_str(text)?;

    let mut next: Vec<Value> = Vec::new();

    next.push(v);

    let mut sum: i64 = 0;

    while let Some(curr) = next.pop() {
        match curr {
            Value::Number(n) => sum += n.as_i64().unwrap(),
            Value::Array(a) => next.extend_from_slice(&a[..]),
            Value::Object(o) => {
                if valid(&o) {
                    for v in o {
                        next.push(v.1);
                    }
                }
            }
            _ => (),
        }
    }
    Ok(sum)
}

fn part1(text: &str) -> u64 {
    let re = Regex::new(r"(-?\d+)").unwrap();

    let mut sum: u64 = 0;
    for cap in re.captures_iter(&text) {
        if cap[1].starts_with('-') {
            sum -= &cap[1][1..].parse().expect("number");
        } else {
            sum += &cap[1].parse().expect("number");
        }
    }
    sum
}

fn main() {
    let input = input(12);
    println!("AoC 2015 Day 12, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 12, Part 2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_correct_answer() {
        assert_eq!(part1(&input(12)), 191164);
    }

    #[test]
    fn day12_part2_correct_answer() {
        assert_eq!(part2(&input(12)).unwrap(), 87842);
    }

}
