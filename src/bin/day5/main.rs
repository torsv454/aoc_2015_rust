extern crate aoc_2015_rust;
use aoc_2015_rust::*;

const VOWELS: &str = "aeiou";
const BAD_WORDS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

fn is_nice(line: &str) -> bool {
    let mut vowel_count = 0;
    let mut twice = false;
    let mut bad_words = true;

    let mut prev = '\n';

    for c in line.chars() {
        if VOWELS.contains(c) {
            vowel_count += 1;
        }

        if !twice && c == prev {
            twice = true;
        }

        bad_words = BAD_WORDS.contains(&(prev, c));

        if bad_words {
            return false;
        }

        prev = c;
    }

    vowel_count > 2 && twice && !bad_words
}

fn is_nice2(line: &str) -> bool {
    let mut c3 = '\n';
    let mut c2 = '\n';
    let mut c1 = '\n';
    let mut repeat = false;
    let mut pairs: Vec<(char, char)> = Vec::new();

    for c in line.chars() {
        if !repeat && c == c2 {
            repeat = true;
        }

        if c1 != '\n' && !(c == c1 && c1 == c2 && c != c3) {
            pairs.push((c1, c));
        }

        c3 = c2;
        c2 = c1;
        c1 = c;
    }

    let len_before = pairs.len();
    pairs.sort();
    pairs.dedup();
    let len_after = pairs.len();

    repeat && len_before > len_after
}

fn part1(input: &str) -> usize {
    input.lines().filter(|x| is_nice(x)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|x| is_nice2(x)).count()
}

fn main() {
    let input = input(5);
    println!("AoC 2015 Day 5, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 5, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_samples() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn day5_part1_correct_answer() {
        assert_eq!(part1(&input(5)), 236);
    }

    fn day5_part2_samples() {
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice2("xxyxx"), true);
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn day5_part2_correct_answer() {
        assert_eq!(part2(&input(5)), 51);
    }
}
