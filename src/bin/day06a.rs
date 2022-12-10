use std::{
    collections::HashSet,
    io::{stdin, Read},
};

// cargo run --bin day06a < input/day06.txt

fn main() {
    let mut buffer: [char; 4] = [' '; 4];
    let mut buffer_set: HashSet<char> = HashSet::new();

    for (i, byte) in stdin().bytes().enumerate() {
        let char = byte.unwrap() as char;
        buffer_set.clear();

        buffer[0] = buffer[1];
        buffer[1] = buffer[2];
        buffer[2] = buffer[3];
        buffer[3] = char;

        if i >= 4 {
            buffer_set.insert(buffer[0]);
            buffer_set.insert(buffer[1]);
            buffer_set.insert(buffer[2]);
            buffer_set.insert(buffer[3]);
        }

        if buffer_set.len() == 4 {
            println!(
                "{}{}{}{}, {}",
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3],
                i + 1
            );
            break;
        }
    }
}
