use std::fs;
use std::collections::HashSet;

fn score(map : &Vec<Vec<u8>>, tree_x : usize, tree_y : usize) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();

    let mut scenic_score : u32 = 1;

    let mut seen : u32 = 0;
    for x in tree_x + 1..map_height {
        seen += 1;
        if (map[x][tree_y]) >= map[tree_x][tree_y] {
            break; // count the tree above, but no more in this direction
        }
    }
    scenic_score *= seen;

    let mut seen : u32 = 0;
    for x in (0..tree_x).rev() {
        seen += 1;
        if (map[x][tree_y]) >= map[tree_x][tree_y] {
            break; // count the tree above, but no more in this direction
        }
    }
    scenic_score *= seen;

    let mut seen : u32 = 0;
    for y in tree_y + 1..map_width {
        seen += 1;
        if (map[tree_x][y]) >= map[tree_x][tree_y] {
            break; // count the tree above, but no more in this direction
        }
    }
    scenic_score *= seen;

    let mut seen : u32 = 0;
    for y in (0..tree_y).rev() {
        seen += 1;
        if (map[tree_x][y]) >= map[tree_x][tree_y] {
            break; // count the tree above, but no more in this direction
        }
    }
    scenic_score *= seen;

    return scenic_score
}

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

    let mut wins : HashSet<(u32, u32)> = HashSet::new();

    let map_width = map[0].len();

    map.iter().enumerate().fold(
        (0..map_width).map(|_| b'0' - 1).collect(),
        |maxes : Vec<u8>, (x, row)| {
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
        |maxes : Vec<u8>, (x, row)| {
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

    println!("total visible from outside: {}", wins.len());

    let max_scenic_score = (
        map.iter().enumerate().map(
            |(x, row)| {
                    row.iter().enumerate().map(
                        |(y, _)| {
                            score(&map, x, y)
                        }
                    ).max()
            }
        ).max()
    ).unwrap().unwrap();
    println!("max_scenic_score: {}", max_scenic_score);
}
