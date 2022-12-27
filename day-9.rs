use std::fs;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;

fn head_move(head: (i32, i32), d_head: (i32, i32)) -> (i32, i32) {
    let (x, y) = head;
    let (dx, dy) = d_head;
    (x + dx, y + dy)
}

fn tail_move(segment: (i32, i32), next_segment: (i32, i32)) -> (i32, i32) {
    let (segment_x, segment_y) = segment;
    let (next_segment_x, next_segment_y) = next_segment;

    let new_next_segment = if
        (segment_x - next_segment_x).abs() == 2 ||
        (segment_y - next_segment_y).abs() == 2
    {
        (
            max(min(segment_x, next_segment_x + 1), next_segment_x - 1),
            max(min(segment_y, next_segment_y + 1), next_segment_y - 1),
        )
    } else {
        (next_segment_x, next_segment_y)
    };

    // println!("{:?} {:?}->{:?}", segment, next_segment, new_next_segment);
    new_next_segment
}

fn main() {
    let file_str = fs::read_to_string("day-9.input").expect("Failed to read file");

    let mut tail_visits : HashSet<(i32, i32)> = HashSet::new();
    let mut neck_visits : HashSet<(i32, i32)> = HashSet::new();

    let start : Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    tail_visits.insert(start[9]);
    neck_visits.insert(start[1]);

    file_str.trim().split("\n").fold(
        start,
        |snake, row| {
            row.split_once(' ').map(
                |(direction, distance)| {
                    match distance.to_string().parse::<i32>() {
                        Ok(distance) => {
                            match direction {
                                "D" => Some((1, 0)),
                                "U" => Some((-1, 0)),
                                "L" => Some((0, -1)),
                                "R" => Some((0, 1)),
                                _ => None,
                            }.map(|head_delta| {
                                (0..distance).fold(
                                    snake,
                                    |snake, _| {
                                        let next_head = head_move(snake[0], head_delta);

                                        // fold through. each iteration, return a more complete
                                        // snake. take the last item from *the last completed
                                        // part-snake* and use it to determine where to move the
                                        // next segment.
                                        let next_snake : Vec<(i32, i32)> = snake[1..].into_iter().fold(
                                            vec![next_head],
                                            |snake_part, &next_segment| {
                                                let &last_segment = snake_part.last().unwrap();
                                                snake_part.into_iter()
                                                .chain(
                                                    vec![tail_move(last_segment, next_segment)].into_iter()//.map(|&a|a)
                                                )
                                                .collect()
                                            }
                                        );

                                        tail_visits.insert(next_snake[9]);
                                        neck_visits.insert(next_snake[1]);
                                        // println!("{:?}", next_snake);

                                        next_snake
                                    }
                                )
                            }).unwrap_or_else(|| {
                                println!("invalid direction {}", direction);
                                Vec::new()
                            })
                        },
                        _ => {
                            println!("invalid distance {}", distance);
                            Vec::new()
                        }
                    }
                }
            ).unwrap_or_else(|| {
                println!("Parsing error for line {}", row);
                Vec::new()
            })
        }
    );
    println!("Tail visits {} locations", tail_visits.len());
    println!("Heck visits {} locations", neck_visits.len());
}
