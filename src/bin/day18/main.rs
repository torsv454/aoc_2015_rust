extern crate aoc_2015_rust;
use aoc_2015_rust::*;

const SIZE: usize = 100;

fn rule(state: usize, neighbours: usize) -> usize {
    if state == 1 {
        if neighbours == 2 || neighbours == 3 {
            1
        } else {
            0
        }
    } else if neighbours == 3 {
        1
    } else {
        state
    }
}

fn count_neighbours(lights: &Vec<usize>, x: i32, y: i32) -> usize {
    let delta: Vec<i32> = vec![-1, 0, 1];

    let mut count = 0;
    for dy in &delta {
        for dx in &delta {
            if dx == dy && *dy == 0 {
                continue;
            }
            if x == 0 && *dx == -1 {
                continue;
            }
            if y == 0 && *dy == -1 {
                continue;
            }
            if x == (SIZE - 1) as i32 && *dx == 1 {
                continue;
            }
            if y == (SIZE - 1) as i32 && *dy == 1 {
                continue;
            }
            let index = (x + dx) + (SIZE as i32) * (y + dy);
            if index >= 0 && index < (lights.len() as i32) {
                count += lights[index as usize];
            }
        }
    }
    count
}

fn step(lights: &Vec<usize>) -> Vec<usize> {
    let mut next = lights.clone();

    for y in 0..(SIZE as i32) {
        for x in 0..(SIZE as i32) {
            let count = count_neighbours(lights, x, y);
            let center = (y * (SIZE as i32) + x) as usize;
            next[center] = rule(lights[center], count);
        }
    }
    next
}

fn step2(lights: &Vec<usize>) -> Vec<usize> {
    let mut next = step(lights);
    turn_on_corners(&mut next);
    next
}

fn turn_on_corners(lights: &mut Vec<usize>) {
    lights[0 * SIZE + 0] = 1;
    lights[(SIZE - 1) * SIZE + 0] = 1;
    lights[0 * SIZE + (SIZE - 1)] = 1;
    lights[(SIZE - 1) * SIZE + (SIZE - 1)] = 1;
}

fn parse(input: &str) -> Vec<usize> {
    let mut result = vec![0; SIZE * SIZE];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result[y * SIZE + x] = 1;
            }
        }
    }

    result
}

fn count_lights_on(lights: &[usize]) -> usize {
    lights.iter().sum()
}

fn part1(input: &str) -> usize {
    let mut lights = parse(input);
    let steps = 100;
    for _ in 0..steps {
        lights = step(&lights);
    }
    count_lights_on(&lights)
}

fn part2(input: &str) -> usize {
    let mut lights = parse(input);
    let steps = 100;
    for _ in 0..steps {
        lights = step2(&lights);
    }
    count_lights_on(&lights)
}

fn main() {
    let input = input(18);
    println!("AoC 2015 Day 18, Part 1: {}", part1(&input));
    println!("AoC 2015 Day 18, Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part1_correct_answer() {
        assert_eq!(part1(&input(18)), 1061);
    }

    #[test]
    fn day18_part2_correct_answer() {
        assert_eq!(part2(&input(18)), 1006);
    }

}
