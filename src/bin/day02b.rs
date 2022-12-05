mod day02a;
use day02a::Hand;
use std::io::stdin;

// cargo run --bin day02b < input/day02.txt

fn score_line(line: String) -> i32 {
    let elf: Option<char>;
    let you: Option<char>;
    {
        let mut chars = line.chars();

        // line should look like "A Y", "B Z", etc.
        elf = chars.next();
        chars.next();
        you = chars.next();
    }

    let elf_hand = match elf {
        Some('A') => Hand::Rock,
        Some('B') => Hand::Paper,
        Some('C') => Hand::Scissors,
        Some(_) => panic!("Invalid move for elf {}", elf.unwrap()),
        None => panic!("Missing move for elf"),
    };
    let (you_hand, outcome_score) = match you {
        // lose
        Some('X') => {
            let losing_hand = match elf_hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            };
            (losing_hand, 0)
        }
        // draw
        Some('Y') => (elf_hand, 3),
        // win
        Some('Z') => (elf_hand.counter(), 6),
        Some(_) => panic!("Invalid move for you {}", you.unwrap()),
        None => panic!("Missing move for you"),
    };

    return you_hand.score() + outcome_score;
}

fn main() {
    let total_score: i32 = stdin().lines().map(|line| score_line(line.unwrap())).sum();

    println!("{}", total_score);
}
