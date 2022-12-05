use std::io::stdin;

// cargo run --bin day02a < input/day02.txt

#[derive(PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: String) -> (Hand, Hand) {
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
    let you_hand = match you {
        Some('X') => Hand::Rock,
        Some('Y') => Hand::Paper,
        Some('Z') => Hand::Scissors,
        Some(_) => panic!("Invalid move for you {}", you.unwrap()),
        None => panic!("Missing move for you"),
    };

    return (elf_hand, you_hand);
}

fn score_hand(hand: &Hand) -> i32 {
    return match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };
}

fn score_outcome(elf: Hand, you: Hand) -> i32 {
    // Draw results in score of 3
    if elf == you {
        return 3;
    }

    // what move would beat the elf's move?
    let winning_move = match elf {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    };

    return if winning_move == you { 6 } else { 0 };
}

fn main() {
    let mut total_score: i32 = 0;
    for line in stdin().lines() {
        let (elf, you) = parse_line(line.unwrap());
        total_score += score_hand(&you) + score_outcome(elf, you);
    }

    println!("{}", total_score);
}
