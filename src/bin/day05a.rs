use regex::Regex;
use std::io::stdin;

// cargo run --bin day04a < input/day04.txt

fn parse_stack(columns: &Vec<usize>, stack: &String) -> Vec<char> {
    let stack_bytes = stack.as_bytes();
    columns.iter().map(|i| stack_bytes[*i] as char).collect()
}

fn parse_stacks(columns: String, stacks: Vec<String>) -> Vec<Vec<char>> {
    let column_nums = columns
        .bytes()
        .enumerate()
        .filter(|(_, char)| (*char as char).is_numeric())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let mut parsed_stacks: Vec<Vec<char>> = (0..column_nums.len()).map(|_| Vec::new()).collect();
    for stack in stacks.iter().rev() {
        let parsed = parse_stack(&column_nums, stack);
        for (j, val) in parsed.iter().enumerate() {
            let value = *val;
            if value != ' ' {
                parsed_stacks[j].push(value);
            }
        }
    }

    return parsed_stacks;
}

struct Move {
    count: i32,
    from: usize,
    to: usize,
}

fn parse_move(line: String, move_re: &Regex) -> Move {
    let captures = move_re.captures(&line).unwrap();
    return Move {
        count: captures.get(1).unwrap().as_str().parse().unwrap(),
        from: captures.get(2).unwrap().as_str().parse().unwrap(),
        to: captures.get(3).unwrap().as_str().parse().unwrap(),
    };
}

fn main() {
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut lines = stdin().lines();

    let stacks_raw = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| line.contains('['))
        .collect::<Vec<String>>();

    let columns = lines.next().unwrap().unwrap();

    if !lines.next().unwrap().unwrap().is_empty() {
        panic!("Expected empty string after {}", columns);
    }

    let mut stacks = parse_stacks(columns, stacks_raw);

    for movement in lines.map(|line| parse_move(line.unwrap(), &move_re)) {
        for _ in 0..movement.count {
            let value = stacks[movement.from].pop().unwrap();
            stacks[movement.to].push(value);
        }
    }

    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", result);
}
