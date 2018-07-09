extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn num_chars(text: &str) -> usize {
    let mut n = 0;
    let mut iter = text[1..text.len() - 1].chars();
    while let Some(c) = iter.next() {
        if c == '\\' {
            match iter.next() {
                Some(nc) => if nc == '\\' || nc == '"' {
                    n += 1;
                } else if nc == 'x' {
                    iter.next();
                    iter.next();
                    n += 1;
                } else {
                    assert!(false);
                },
                None => assert!(false),
            }
        } else {
            n += 1;
        }
    }
    n
}

fn encode(text: &str) -> String {
    let mut encoded = String::new();
    encoded.push('"');
    for c in text.chars() {
        if c == '"' || c == '\\' {
            encoded.push('\\');
        }
        encoded.push(c);
    }
    encoded.push('"');
    encoded
}

fn parts(input: &str) -> (usize, usize) {
    let mut char_count = 0;
    let mut data_count = 0;
    let mut encoded_count = 0;

    for line in input.lines() {
        char_count += line.len();
        data_count += num_chars(&line);
        encoded_count += encode(&line).len()
    }

    let part1 = char_count - data_count;
    let part2 = encoded_count - char_count;

    (part1, part2)
}

fn main() {
    let parts = parts(&input(8));
    println!("AoC 2015 Day 8, Part 1: {}", parts.0);
    println!("AoC 2015 Day 8, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_samples() {
        assert_eq!(0, num_chars("\"\""), "1");
        assert_eq!(3, num_chars("\"abc\""), "2");
        assert_eq!(7, num_chars("\"aaa\\\"aaa\""), "3");
        assert_eq!(1, num_chars("\"\\x27\""), "4");
    }

    #[test]
    fn day8_both_parts_correct() {
        let parts = parts(&input(8));
        assert_eq!(parts.0, 1350);
        assert_eq!(parts.1, 2085);
    }

    #[test]
    fn day8_part2_samples() {
        assert_eq!("\"\\\"\\\"\"", encode("\"\""));
        assert_eq!("\"\\\"abc\\\"\"", encode("\"abc\""));
        assert_eq!("\"\\\"aaa\\\\\\\"aaa\\\"\"", encode("\"aaa\\\"aaa\""));
    }
}
