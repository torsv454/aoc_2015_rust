use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Change {
    left: String,
    right: String,
    change: i64,
}

fn parse_line(line: &str) -> Change {
    let parts: Vec<&str> = line.split(' ').collect();
    let left = parts[0];
    let dir = parts[2];
    let right = parts[10];
    let units = parts[3].parse().unwrap();

    Change {
        left: left.to_string(),
        right: right[0..right.len() - 1].to_string(),
        change: if dir == "gain" { units } else { -units },
    }
}

fn parse(filename: &str) -> Vec<Change> {
    let f = File::open(filename).expect("no such file");
    let mut v = Vec::new();
    for line in BufReader::new(&f).lines() {
        v.push(parse_line(&line.unwrap()));
    }
    v
}

fn permute(len: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();

    res.push(vec![0]);

    for i in 1..len {
        let mut nres = Vec::new();
        for v in res {
            for j in 0..v.len() + 1 {
                let mut nv = v.clone();
                nv.insert(j, i);
                nres.push(nv);
            }
        }
        res = nres;
    }

    res
}

fn arrange_seats(size: usize, cost: &Vec<i64>) -> i64 {
    //    let size = (cost.len() as f64).sqrt() as usize + 1;
    let permutations = permute(size);
    let mut max = std::i64::MIN;
    for permutation in &permutations {
        let mut prev = permutation.last().unwrap();
        let mut change: i64 = 0;
        for index in permutation {
            change += cost[prev * size + index];
            change += cost[index * size + prev];
            prev = index;
        }
        if change > max {
            max = change;
        }
    }
    max
}

fn build_table(
    size: usize,
    changes: &Vec<Change>,
    people_index: &HashMap<String, usize>,
) -> Vec<i64> {
    let mut cost: Vec<i64> = vec![0; size * size];

    for c in changes {
        let i = people_index.get(&c.left).unwrap();
        let j = people_index.get(&c.right).unwrap();
        cost[i * size + j] = c.change;
    }

    cost
}

fn parts() -> (i64, i64) {
    let changes: Vec<Change> = parse("src/bin/day13/input.txt");

    let mut people_index: HashMap<String, usize> = HashMap::new();
    let mut n = 0;
    for c in &changes {
        if !people_index.contains_key(&c.left) {
            people_index.insert(c.left.clone(), n);
            n += 1;
        }
        if !people_index.contains_key(&c.right) {
            people_index.insert(c.right.clone(), n);
            n += 1;
        }
    }

    let size = people_index.len();

    let cost1 = build_table(size, &changes, &people_index);

    let part1 = arrange_seats(size, &cost1);

    let cost2 = build_table(size + 1, &changes, &people_index);

    let part2 = arrange_seats(size + 1, &cost2);

    (part1, part2)
}

fn main() {
    let parts = parts();
    println!("AoC 2015 Day 13, Part 1: {}", parts.0);
    println!("AoC 2015 Day 13, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_both_parts_correct() {
        let parts = parts();
        assert_eq!(parts.0, 709);
        assert_eq!(parts.1, 668);
    }
}
