extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn pset(elements: &[usize]) -> Vec<Vec<usize>> {
    let mut set: Vec<Vec<usize>> = Vec::new();
    set.push(Vec::new());

    for element in elements {
        let mut new_set: Vec<Vec<usize>> = Vec::new();
        for member in &set {
            let mut newmember = member.clone();
            newmember.push(*element);
            new_set.push(newmember);
        }
        set.extend(new_set);
    }

    set
}

fn valid_candidates(input: &str) -> Vec<Vec<usize>> {
    let input: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let candidates = pset(&input);
    candidates
        .iter()
        .filter(|set| {
            let sum: usize = set.iter().sum();
            sum == 150
        })
        .cloned()
        .collect()
}

fn part1(input: &str) -> usize {
    valid_candidates(input).len()
}

fn part2(input: &str) -> usize {
    let valid = valid_candidates(input);
    let num_containers = valid
        .iter()
        .map(|x| x.len())
        .fold(std::usize::MAX, std::cmp::min);

    valid
        .iter()
        .map(|x| x.len())
        .filter(|x| *x == num_containers)
        .count()
}

fn main() {
    let input = input(17);
    println!("AoC 2015 Day 17, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 17, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_part1_correct_answer() {
        assert_eq!(part1(&input(17)), 1638);
    }

    #[test]
    fn day17_part2_correct_answer() {
        assert_eq!(part2(&input(17)), 17);
    }

}
