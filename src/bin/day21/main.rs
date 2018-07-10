extern crate aoc_2015_rust;
use aoc_2015_rust::*;

#[derive(Clone, Debug)]
struct Character {
    hp: i32,
    armor: i32,
    damage: i32,
}

#[derive(PartialEq, Debug)]
enum CharacterType {
    Player,
    Boss,
}

#[derive(Clone, Debug, PartialEq)]
struct Item {
    name: String,
    cost: i32,
    armor: i32,
    damage: i32,
}

#[derive(Debug)]
struct Shop(Vec<Item>, Vec<Item>, Vec<Item>);

const CHARACTER_HITPOINTS: &str = "Hit Points";
const CHARACTER_DAMAGE: &str = "Damage";
const CHARACTER_ARMOR: &str = "Armor";
const CHARACTER_SPLIT: &str = ": ";

const ITEM_WEAPONS: &str = "Weapons:";
const ITEM_RINGS: &str = "Rings:";
const ITEM_ARMOR: &str = "Armor:";
const ITEM_SPLIT: &str = " ";

fn parse_character(filename: &str) -> Character {
    let mut hp = 0;
    let mut armor = 0;
    let mut damage = 0;

    for line in slurp(filename).lines() {
        let parts: Vec<&str> = line.split(CHARACTER_SPLIT).collect();
        let attr = parts[0];
        let value = parts[1].parse().unwrap();
        match attr {
            CHARACTER_HITPOINTS => hp = value,
            CHARACTER_ARMOR => armor = value,
            CHARACTER_DAMAGE => damage = value,
            _ => (),
        }
    }
    Character { hp, armor, damage }
}

fn parse_items(filename: &str) -> Shop {
    let contents: String = slurp(filename);
    let mut result = Shop(Vec::new(), Vec::new(), Vec::new());

    //result.0.push(Item {name: "<empty>".into(), cost: 0, armor: 0, damage: 0});
    result.1.push(Item {
        name: "<empty>".into(),
        cost: 0,
        armor: 0,
        damage: 0,
    });
    result.2.push(Item {
        name: "<empty>".into(),
        cost: 0,
        armor: 0,
        damage: 0,
    });

    let mut mode = "";
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(ITEM_SPLIT).filter(|x| x.len() != 0).collect();
        if parts.len() != 4 && parts.len() != 5 {
            continue;
        }
        if Some(':') == parts[0].chars().last() {
            mode = parts[0];
        } else {
            let name = parts[0].to_string();
            let offset = if mode == ITEM_RINGS { 1 } else { 0 };
            let cost = parts[1 + offset].parse().unwrap();
            let damage = parts[2 + offset].parse().unwrap();
            let armor = parts[3 + offset].parse().unwrap();
            let item = Item {
                name,
                cost,
                armor,
                damage,
            };
            match mode {
                ITEM_WEAPONS => result.0.push(item),
                ITEM_ARMOR => result.1.push(item),
                ITEM_RINGS => result.2.push(item),
                _ => panic!("unknown item"),
            };
        }
    }
    result
}

fn fight(player: &mut Character, boss: &mut Character) -> CharacterType {
    loop {
        let damage = std::cmp::max(1, player.damage - boss.armor);
        boss.hp -= damage;
        //println!("Player deals {}, boss has {}.",damage,boss.hp);
        if boss.hp <= 0 {
            return CharacterType::Player;
        }

        let damage = std::cmp::max(1, boss.damage - player.armor);
        player.hp -= damage;
        //println!("Boss deals {}, player has {}.",damage,player.hp);
        if player.hp <= 0 {
            return CharacterType::Boss;
        }
    }
}

fn equipment_combos(shop: &Shop) -> Vec<Vec<&Item>> {
    let mut result = Vec::new();

    for weapon in &shop.0 {
        for armor in &shop.1 {
            for left_ring in &shop.2 {
                for right_ring in &shop.2 {
                    if left_ring == right_ring {
                        continue;
                    }
                    let mut combo = Vec::new();
                    combo.push(weapon);
                    combo.push(armor);
                    combo.push(left_ring);
                    combo.push(right_ring);
                    result.push(combo);
                }
            }
        }
    }

    result
}

fn wins_with_equipment(
    player_template: &Character,
    boss_template: &Character,
    equipment: &Vec<&Item>,
) -> bool {
    let mut player = player_template.clone();
    let mut boss = boss_template.clone();

    for item in equipment {
        player.damage += item.damage;
        player.armor += item.armor;
    }

    CharacterType::Player == fight(&mut player, &mut boss)
}

fn parts() -> (i32, i32) {
    let player_template = Character {
        hp: 100,
        armor: 0,
        damage: 0,
    };
    let boss_template = parse_character("src/bin/day21/boss.txt");
    let shop = parse_items("src/bin/day21/shop.txt");
    let combos = equipment_combos(&shop);

    let winning_combos: Vec<&Vec<&Item>> = combos
        .iter()
        .filter(|equipment| wins_with_equipment(&player_template, &boss_template, equipment))
        .collect();

    let mut min_cost = std::i32::MAX;

    winning_combos
        .iter()
        .for_each(|x| min_cost = std::cmp::min(min_cost, x.iter().map(|e| e.cost).sum()));

    let loosing_combos: Vec<&Vec<&Item>> = combos
        .iter()
        .filter(|equipment| !wins_with_equipment(&player_template, &boss_template, equipment))
        .collect();

    let mut costs: Vec<i32> = loosing_combos
        .iter()
        .map(|x| x.iter().map(|e| e.cost).sum::<i32>())
        .collect();
    costs.sort();
    costs.dedup();

    let mut max_cost = std::i32::MIN;

    loosing_combos
        .iter()
        .for_each(|x| max_cost = std::cmp::max(max_cost, x.iter().map(|e| e.cost).sum()));

    (min_cost, max_cost)
}

fn main() {
    let parts = parts();
    println!("AoC 2015 Day 21, Part 1: {}", parts.0);
    println!("AoC 2015 Day 21, Part 2: {}", parts.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_both_parts_correct_answer() {
        let parts = parts();
        assert_eq!(parts.0, 121);
        assert_eq!(parts.1, 201);
    }

}
