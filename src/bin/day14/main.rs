extern crate aoc_2015_rust;
use aoc_2015_rust::*;

const TIME_LIMIT: u32 = 2503;

struct Reindeer {
    speed: u32,
    travel_for: u32,
    rest_for: u32,
    points: u32,
    distance: u32,
}

fn distance(r: &Reindeer, t: u32) -> u32 {
    let cycle_time = r.travel_for + r.rest_for;
    let cycles = t / cycle_time;
    let remaining_time = t - (cycles * cycle_time);
    let remaining_flight_time = std::cmp::min(r.travel_for, remaining_time);
    let distance = cycles * r.travel_for * r.speed + remaining_flight_time * r.speed;
    distance
}

fn parse(line: &str) -> Reindeer {
    let parts: Vec<&str> = line.split(' ').collect();
    let speed = parts[3].parse().unwrap();
    let travel_for = parts[6].parse().unwrap();
    let rest_for = parts[13].parse().unwrap();

    Reindeer {
        speed,
        travel_for,
        rest_for,
        points: 0,
        distance: 0,
    }
}

fn part1(input: &str, time_limit: u32) -> u32 {
    let reindeers: Vec<Reindeer> = input.lines().map(parse).collect();
    let mut max = 0;
    for reindeer in &reindeers {
        max = std::cmp::max(max, distance(&reindeer, time_limit));
    }

    max
}

fn part2(input: &str, time_limit: u32) -> u32 {
    let mut reindeers: Vec<Reindeer> = input.lines().map(parse).collect();

    for i in 1..=time_limit {
        let mut max = 0;
        for reindeer in &mut reindeers {
            reindeer.distance = distance(reindeer, i);
            max = std::cmp::max(max, reindeer.distance);
        }

        for reindeer in &mut reindeers {
            if reindeer.distance == max {
                reindeer.points += 1;
            }
        }
    }
    reindeers.iter().fold(0, |r, n| std::cmp::max(r, n.points))
}

fn main() {
    let input = input(14);
    println!("AoC 2015 Day 14, Part 1: {}", part1(&input, TIME_LIMIT));
    println!("AoC 2015 Day 14, Part 2: {}", part2(&input, TIME_LIMIT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1_correct_answer() {
        assert_eq!(part1(&input(14, TIME_LIMIT)), 2696);
    }

    #[test]
    fn day14_part2_correct_answer() {
        assert_eq!(part2(&input(14, TIME_LIMIT)), 1084);
    }

}
