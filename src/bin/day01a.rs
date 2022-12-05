use std::cmp::max;
use std::io::stdin;

// cargo run --bin day01a < input/day01a.txt

fn main() {
    let mut largest_so_far: i32 = 0;
    let mut current_elf_calories: i32 = 0;
    for line in stdin().lines() {
        let input = line.unwrap();
        if input.is_empty() {
            largest_so_far = max(largest_so_far, current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories: i32 = input.parse().expect("Input not an integer");
            current_elf_calories += calories;
        }
    }

    println!("{}", largest_so_far);
}
