extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn part1(input: &str) -> usize {
    let limit = input.parse().unwrap();

    let mut houses = vec![10; 1_000_000];
    for i in 2..999_999 {
        let mut j = i;
        while j < houses.len() {
            houses[j - 1] += i * 10;
            j += i;
        }
    }

    let result = houses
        .iter()
        .enumerate()
        .filter(|(_i, x)| **x >= limit)
        .next();

    result.unwrap().0 + 1
}

fn part2(input: &str) -> usize {
    let limit = input.parse().unwrap();
    let mut houses = vec![10; 1_000_000];
    for i in 2..999_999 {
        let mut j = i;
        let mut count = 0;
        while j < houses.len() && count < 50 {
            houses[j - 1] += i * 11;
            j += i;
            count += 1;
        }
    }

    let result = houses
        .iter()
        .enumerate()
        .filter(|(_i, x)| **x >= limit)
        .next();

    result.unwrap().0 + 1
}

fn main() {
    let input = input(20);
    println!("AoC 2015 Day 20, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 20, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part1_correct_answer() {
        assert_eq!(part1(&input(20)), 776160);
    }

    #[test]
    fn day20_part2_correct_answer() {
        assert_eq!(part2(&input(20)), 786240);
    }

}
