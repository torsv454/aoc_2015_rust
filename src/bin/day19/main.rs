extern crate aoc_2015_rust;
use aoc_2015_rust::*;

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

fn molecule_from_string(text: &str) -> Vec<String> {
    let mut element = String::new();
    let mut result = Vec::new();
    for c in text.chars() {
        if c.is_uppercase() {
            if element.len() > 0 {
                result.push(element);
                element = String::new();
            }
            element.push(c);
        } else {
            element.push(c);
            result.push(element);
            element = String::new();
        }
    }
    if element.len() > 0 {
        result.push(element);
    }
    result
}

fn replacements(replacement: &Replacement, molecule: &Vec<String>) -> Vec<Vec<String>> {
    let mut replacements = Vec::new();
    for (index, element) in molecule.iter().enumerate() {
        if replacement.from == *element {
            let mut new_molecule = molecule.clone();
            new_molecule[index] = replacement.to.clone();
            replacements.push(new_molecule);
        }
    }
    replacements
}

fn parse(input: &str) -> (Vec<Replacement>, Vec<String>) {
    let mut replacements = Vec::new();
    let molecule = molecule_from_string(input.lines().last().unwrap());

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            replacements.push(Replacement {
                from: parts[0].into(),
                to: parts[1].into(),
            });
        }
    }

    (replacements, molecule)
}

fn part1(input: &str) -> usize {
    // Brute force approach
    let (repls, starting_molecule) = parse(input);

    let mut candidate_molecules = Vec::new();

    repls.iter().for_each(|r| {
        replacements(r, &starting_molecule)
            .iter()
            .for_each(|x| candidate_molecules.push(x.join("")))
    });

    candidate_molecules.sort();
    candidate_molecules.dedup();
    candidate_molecules.len()
}

fn part2(input: &str) -> usize {
    // TODO...
    0
}

fn main() {
    let input = input(19);
    println!("AoC 2015 Day 19, Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part1_correct_answer() {
        assert_eq!(part1(&input(19)), 576);
    }
}
