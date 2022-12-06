use std::{io::stdin, ops::Range};

// cargo run --bin day04a < input/day04.txt

fn parse_part(part: String) -> Range<i32> {
    let mut nums = part.split('-');
    let start = nums.next().unwrap().parse::<i32>().unwrap();
    let end = nums.next().unwrap().parse::<i32>().unwrap();
    return start..end;
}

fn parse_line(line: String) -> (Range<i32>, Range<i32>) {
    let mut parts = line.split(',');
    return (
        parse_part(parts.next().unwrap().to_string()),
        parse_part(parts.next().unwrap().to_string()),
    );
}

fn ranges_fully_overlap(ranges: &(Range<i32>, Range<i32>)) -> bool {
    let (range1, range2) = ranges;
    return range1.start <= range2.start && range1.end >= range2.end
        || range2.start <= range1.start && range2.end >= range1.end;
}

fn main() {
    let total_score: i32 = stdin()
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .filter(|ranges| ranges_fully_overlap(ranges))
        .count() as i32;

    println!("{}", total_score);
}
