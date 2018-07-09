extern crate aoc_2015_rust;
use aoc_2015_rust::*;

use std::collections::HashMap;

#[derive(Debug)]
struct Road {
    from: String,
    to: String,
    cost: u32,
}

fn parse(line: &str) -> Road {
    let parts: Vec<&str> = line.split(' ').collect();
    let from = parts[0].to_string();
    let to = parts[2].to_string();
    let cost: u32 = parts[4].parse().expect("invalid number");

    Road { from, to, cost }
}

fn permutations(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    result.push(vec![0]);
    for i in 1..n {
        let mut next_result = Vec::new();
        for v in &result {
            for j in 0..v.len() + 1 {
                let mut nv = v.clone();
                nv.insert(j, i);
                next_result.push(nv);
            }
        }
        result = next_result;
    }
    result
}

fn cities(roads: &[Road]) -> HashMap<String, usize> {
    let mut cities = HashMap::new();
    let mut n = 0;
    for road in roads {
        if !cities.contains_key(&road.from) {
            cities.insert(road.from.clone(), n);
            n += 1;
        }
        if !cities.contains_key(&road.to) {
            cities.insert(road.to.clone(), n);
            n += 1;
        }
    }
    cities
}

fn costs(roads: &[Road], cities: &HashMap<String, usize>) -> Vec<u32> {
    let num_cities = cities.len();
    let mut cost = vec![std::u32::MAX; num_cities * num_cities];

    for road in roads {
        let x = cities.get(&road.from).expect("missing city");
        let y = cities.get(&road.to).expect("missing city");

        cost[x * num_cities + y] = road.cost;
        cost[y * num_cities + x] = road.cost;
    }
    cost
}

fn parts(input: &str) -> (u32, u32) {
    let roads: Vec<Road> = input.lines().map(parse).collect();
    let cities = cities(&roads);
    let num_cities = cities.len();
    let cost = costs(&roads, &cities);
    let paths = permutations(num_cities);

    let mut min_cost = std::u32::MAX;
    let mut max_cost = std::u32::MIN;
    for path in paths {
        let mut path_cost = 0;
        let mut curr = &path[0];
        for next in &path[1..] {
            path_cost += cost[curr * num_cities + next];
            curr = next;
        }
        if path_cost < min_cost {
            min_cost = path_cost;
        }
        if path_cost > max_cost {
            max_cost = path_cost;
        }
    }

    (min_cost, max_cost)
}

fn main() {
    let parts = parts(&input(9));
    println!("AoC 2015 Day 9, Part 1: {}", parts.0);
    println!("AoC 2015 Day 9, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_both_parts_correct() {
        let parts = parts(&input(9));
        assert_eq!(parts.0, 251);
        assert_eq!(parts.1, 898);
    }

}
