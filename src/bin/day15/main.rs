extern crate aoc_2015_rust;
use aoc_2015_rust::*;

fn combinations(elements: usize, size: usize) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = Vec::new();
    let values: Vec<usize> = (1..=(size - elements + 1)).collect();

    for n in 0..elements {
        let mut nres: Vec<Vec<usize>> = Vec::new();
        if n == 0 {
            for v in &values {
                nres.push(vec![*v]);
            }
        } else {
            for r in &res {
                for v in &values {
                    let mut nr: Vec<usize> = r.clone();
                    nr.push(*v);
                    nres.push(nr);
                }
            }
        }
        res = nres;
    }

    res = res.into_iter()
        .filter(|x| x.iter().map(|x| *x).sum::<usize>() == size)
        .collect();

    res
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_ingredient(line: &str) -> Ingredient {
    let parts: Vec<&str> = line.split(' ').collect();
    let name = parts[0][0..parts[0].len() - 1].into();
    let capacity = parts[2][..parts[2].len() - 1].parse().unwrap();
    let durability = parts[4][..parts[4].len() - 1].parse().unwrap();
    let flavor = parts[6][..parts[6].len() - 1].parse().unwrap();
    let texture = parts[8][..parts[8].len() - 1].parse().unwrap();
    let calories = parts[10].parse().unwrap();

    Ingredient {
        name,
        capacity,
        durability,
        flavor,
        texture,
        calories,
    }
}

fn evaluate(ingredients: &[Ingredient], amounts: &[usize]) -> (i64, i64) {
    let mut r = (0, 0, 0, 0);
    let mut calories = 0;

    for i in 0..amounts.len() {
        r.0 += amounts[i] as i64 * ingredients[i].capacity;
        r.1 += amounts[i] as i64 * ingredients[i].durability;
        r.2 += amounts[i] as i64 * ingredients[i].flavor;
        r.3 += amounts[i] as i64 * ingredients[i].texture;
        calories += amounts[i] as i64 * ingredients[i].calories;
    }

    if r.0 < 0 || r.1 < 0 || r.2 < 0 || r.3 < 0 {
        (0, calories)
    } else {
        (r.0 * r.1 * r.2 * r.3, calories)
    }
}

fn parts(input: &str) -> (i64, i64) {
    let ingredients: Vec<Ingredient> = input.lines().map(parse_ingredient).collect();

    let size = 100;
    let combos = combinations(ingredients.len(), size);

    let mut part1 = 0;
    let mut part2 = 0;
    for combo in &combos {
        let (score, calories) = evaluate(&ingredients, combo);

        part1 = std::cmp::max(part1, score);
        part2 = std::cmp::max(part2, if calories == 500 { score } else { 0 });
    }
    (part1, part2)
}

fn main() {
    let parts = parts(&input(15));
    println!("AoC 2015 Day 15, Part 1: {}", parts.0);
    println!("AoC 2015 Day 15, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_both_parts_correct_answer() {
        let parts = parts(&input(15));
        assert_eq!(parts.0, 222870);
        assert_eq!(parts.1, 117936);
    }

}
