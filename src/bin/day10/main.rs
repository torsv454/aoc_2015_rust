extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn next(input: &str) -> String {
    let mut result = String::new();
    let chars = input.chars();
    let mut last = '\n';
    let mut count = 1;
    for c in chars {
        if c == last {
            count += 1;
        } else {
            if last != '\n' {
                result.push_str(&count.to_string());
                result.push_str(&last.to_string());
            }
            last = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push_str(&last.to_string());

    result
}

fn parts(input: String) -> (usize, usize) {
    let mut parts = vec![0; 2];
    let mut curr = input.clone();
    for i in 0..50 {
        curr = next(&curr);
        if i == 39 {
            parts[0] = curr.len();
        } else if i == 49 {
            parts[1] = curr.len();
        }
    }
    (parts[0], parts[1])
}

fn main() {
    let parts = parts(input(10));
    println!("AoC 2015 Day 10, Part 1: {}", parts.0);
    println!("AoC 2015 Day 10, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Too slow :-(
    //#[test]
    //fn day10_both_parts_correct() {
    //    let parts = parts(input(9));
    //    assert_eq!(parts.0, 492982);
    //    assert_eq!(parts.1, 6989950);
    //}

}
