extern crate aoc_2015_rust;
use aoc_2015_rust::*;
use std::str;

const A: u8 = 97;
const Z: u8 = 122;
const I: u8 = 'i' as u8;
const L: u8 = 'l' as u8;
const O: u8 = 'o' as u8;

fn next(pwd: &mut [u8]) {
    for i in (0..pwd.len()).rev() {
        pwd[i] += 1;
        if pwd[i] > Z {
            pwd[i] = A;
        } else {
            break;
        }
    }
}

fn valid(pwd: &[u8]) -> bool {
    let mut seq = false;
    let mut seq_count = 0;
    let mut last = 0;
    let mut pair_count = 0;
    let mut used_in_pair = false;

    for c in pwd {
        if !seq {
            if *c == (last + 1) {
                seq_count += 1;
                if seq_count == 2 {
                    seq = true;
                }
            } else {
                seq_count = 0;
            }
        }

        if used_in_pair {
            used_in_pair = false;
        } else if *c == last {
            pair_count += 1;
            used_in_pair = true;
        }

        last = *c;

        if last == I || last == O || last == L {
            return false;
        }
    }

    seq && pair_count > 1
}

fn next_pwd(pwd: &mut [u8]) {
    loop {
        next(pwd);
        if valid(pwd) {
            return;
        }
    }
}

fn parts(input: &str) -> (String, String) {
    let mut arr: Vec<u8> = input.as_bytes().iter().map(|x| *x).collect();

    next_pwd(&mut arr);
    let part1 = str::from_utf8(&arr).unwrap().to_string();

    next_pwd(&mut arr);
    let part2 = str::from_utf8(&arr).unwrap().to_string();

    (part1, part2)
}

fn main() {
    let parts = parts(&input(11));
    println!("AoC 2015 Day 11, Part 1: {}", parts.0);
    println!("AoC 2015 Day 11, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_both_parts_correct() {
        let parts = parts(&input(11));
        assert_eq!(parts.0, "hxbxxyzz");
        assert_eq!(parts.1, "hxcaabcc");
    }
}
