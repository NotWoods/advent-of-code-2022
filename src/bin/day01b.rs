use std::collections::BinaryHeap;
use std::io::stdin;

// cargo run --bin day01b < input/day01.txt

fn main() {
    let mut largest = BinaryHeap::new();
    let mut current_elf_calories: i32 = 0;
    for line in stdin().lines() {
        let input = line.unwrap();
        if input.is_empty() {
            largest.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories: i32 = input.parse().expect("Input not an integer");
            current_elf_calories += calories;
        }
    }

    let mut top_n_calories_sum = 0;
    for _ in 0..3 {
        top_n_calories_sum += largest.pop().unwrap();
    }

    println!("{}", top_n_calories_sum);
}
