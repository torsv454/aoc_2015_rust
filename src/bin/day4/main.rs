extern crate aoc_2015_rust;
extern crate crypto;

use aoc_2015_rust::*;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn mine(key: &str, start_at: u64, prefix: &str) -> u64 {
    let mut hasher = Md5::new();
    for i in start_at.. {
        let input = format!("{}{}", key, i);
        hasher.reset();
        hasher.input_str(input.as_str());
        let result = hasher.result_str();
        if result.starts_with(prefix) {
            return i;
        }
    }
    return 0;
}

fn part1(key: &str) -> u64 {
    mine(key, 1, "00000")
}

fn part2(key: &str, start_at: u64) -> u64 {
    mine(key, start_at, "000000")
}

fn main() {
    let input = input(3);
    let part1 = part1(&input);
    println!("AoC 2015 Day 4, Part 1: {}", part1);
    println!("AoC 2015 Day 4, Part 2: {}", part2(&input, part1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_samples() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }

    #[test]
    fn day4_part1_correct_answer() {
        assert_eq!(part1(&input(4)), 254575);
    }

    #[test]
    fn day4_part2_correct_answer() {
        assert_eq!(part2(&input(4), 254575), 1038736);
    }

}
