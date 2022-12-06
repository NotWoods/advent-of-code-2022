use std::{collections::HashSet, io::stdin};

// cargo run --bin day03b < input/day03.txt

const LOWER_A_CHARCODE: i32 = 'a' as i32;
const UPPER_A_CHARCODE: i32 = 'A' as i32;

fn priority(char: char) -> i32 {
    if char.is_ascii_alphabetic() {
        if char.is_ascii_lowercase() {
            let charcode = (char as i32) - LOWER_A_CHARCODE;
            return charcode + 1;
        } else {
            let charcode = (char as i32) - UPPER_A_CHARCODE;
            return charcode + 27;
        }
    } else {
        panic!("Invalid item {}", char);
    }
}

fn priorities_for_line(line: String) -> HashSet<i32> {
    let mut set = HashSet::new();

    for char in line.chars() {
        set.insert(priority(char));
    }

    return set;
}

fn main() {
    let mut total_score = 0;

    let mut common_items: HashSet<i32> = HashSet::new();

    let mut i = 0;
    for line in stdin().lines() {
        if i == 0 {
            common_items = priorities_for_line(line.unwrap());
            i = 1;
        } else if i == 1 {
            common_items = common_items
                .intersection(&priorities_for_line(line.unwrap()))
                .cloned()
                .collect();
            i = 2;
        } else {
            let mut only_match: Option<i32> = None;
            for item in priorities_for_line(line.unwrap()) {
                if common_items.contains(&item) {
                    only_match = Some(item);
                    break;
                }
            }

            total_score += only_match.unwrap();

            i = 0;
        }
    }

    println!("{}", total_score);
}
