extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn part1(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold(0, |f, d| f + d)
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    let mut part2 = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 && part2 == 0 {
            part2 = i + 1;
        }
    }
    part2
}

fn main() {
    let input = input(1);
    println!("AoC 2015 Day 1, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 1, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_samples() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn day1_part1_correct_answer() {
        // Correct answer
        assert_eq!(part1(&input(1)), 138);
    }

    #[test]
    fn day1_part2_samples() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }

    #[test]
    fn day1_part2_correct_answer() {
        assert_eq!(part2(&input(1)), 1771);
    }

}
