use std::fs;
use std::collections::HashSet;

fn main() {
    let file_str = fs::read_to_string("day-8.input").expect("Failed to read file");

    let mut map : Vec<Vec<u32>> = Vec::new();

    for line in file_str.trim().split('\n') {
        let mut row = Vec::new();
        for height in line.bytes() {
            row.push(height);
        }
        map.push(row);
    }

    let mut wins : HashSet<(u32, u32)> = HashSet::new();

    let map_width = map[0].len();

    map.iter().enumerate().fold(
        (0..map_width).map(|_| b'0' - 1).collect(),
        |maxes : Vec<u32>, (x, row)| {
            row.iter().zip(maxes).enumerate().map(|(y, (&height, prev_max))| {
                if height > prev_max {
                    wins.insert((x as u32, y as u32));
                    return height;
                }
                return prev_max;
            }).collect()
        }
    );

    map.iter().enumerate().rev().fold(
        (0..map_width).map(|_| b'0' - 1).collect(),
        |maxes : Vec<u32>, (x, row)| {
            row.iter().zip(maxes).enumerate().map(|(y, (&height, prev_max))| {
                if height > prev_max {
                    wins.insert((x as u32, y as u32));
                    return height;
                }
                return prev_max;
            }).collect()
        }
    );

    map.iter().enumerate().map(
        |(x, row)| {
            row.iter().enumerate().fold(b'0' - 1, |prev_max, (y, &height)| {
                if height > prev_max {
                    wins.insert((x as u32, y as u32));
                    return height;
                }
                return prev_max;
            })
        }
    ).for_each(drop);

    map.iter().enumerate().map(
        |(x, row)| {
            row.iter().enumerate().rev().fold(b'0' - 1, |prev_max, (y, &height)| {
                if height > prev_max {
                    wins.insert((x as u32, y as u32));
                    return height;
                }
                return prev_max;
            })
        }
    ).for_each(drop);

    println!("total: {}", wins.len())
}
