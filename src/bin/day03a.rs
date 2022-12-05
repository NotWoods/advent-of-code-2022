use std::{collections::HashSet, io::stdin};

// cargo run --bin day03a < input/day03.txt

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

fn priorities_for_line(line: String) -> i32 {
    let compartment_size = line.len() / 2;
    let mut chars = line.chars();
    let mut set = HashSet::new();

    for _ in 0..compartment_size {
        let char = chars.next().unwrap();
        set.insert(priority(char));
    }
    for char in chars {
        let item_priority = priority(char);
        if set.contains(&item_priority) {
            return item_priority;
        }
    }

    return 0;
}

fn main() {
    let total_score: i32 = stdin()
        .lines()
        .map(|line| priorities_for_line(line.unwrap()))
        .sum();

    println!("{}", total_score);
}
