use std::fs;
use std::collections::HashSet;

fn main() {
    let file_str = fs::read_to_string("day-8.input").expect("Failed to read file");

    let mut map : Vec<Vec<u8>> = Vec::new();

    for line in file_str.trim().split('\n') {
        let mut row = Vec::new();
        for height in line.bytes() {
            row.push(height);
        }
        map.push(row);
    }

    let mut wins : HashSet<(u8, u8)> = HashSet::new();

    map.iter().enumerate().fold(
        (0..map.len()).map(|_| b'0' - 1).collect(),
        |maxes : Vec<u8>, (x, row)| {
            row.iter().zip(maxes).enumerate().map(|(y, (&height, prev_max))| {
                if height > prev_max {
                    wins.insert((x as u8, y as u8));
                    return height;
                }
                return prev_max;
            }).collect()
        }
    );

    map.iter().enumerate().fold(
        (0..map.len()).map(|_| b'0' - 1).collect(),
        |maxes : Vec<u8>, (x, row)| {
            row.iter().zip(maxes).enumerate().rev().map(|(y, (&height, prev_max))| {
                if height > prev_max {
                    wins.insert((x as u8, y as u8));
                    return height;
                }
                return prev_max;
            }).collect()
        }
    );

    println!("{}", wins.len())
}
