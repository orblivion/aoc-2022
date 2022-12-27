use std::fs;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;

fn head_move(head: (i32, i32), d_head: (i32, i32)) -> (i32, i32) {
    let (x, y) = head;
    let (dx, dy) = d_head;
    (x + dx, y + dy)
}

fn tail_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (head_x, head_y) = head;
    let (tail_x, tail_y) = tail;

    let new_tail = if (head_x - tail_x).abs() == 2 || (head_y - tail_y).abs() == 2 {
        (
            max(min(head_x, tail_x + 1), tail_x - 1),
            max(min(head_y, tail_y + 1), tail_y - 1),
        )
    } else {
        (tail_x, tail_y)
    };

    // println!("{:?} {:?}->{:?}", head, tail, new_tail);
    new_tail
}

fn main() {
    let file_str = fs::read_to_string("day-9.input").expect("Failed to read file");

    let mut tail_visits : HashSet<(i32, i32)> = HashSet::new();

    let start = (0, 0);
    tail_visits.insert(start);

    file_str.trim().split("\n").fold(
        (start, start),
        |(head, tail), row| {
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
                                    (head, tail),
                                    |(head, tail), _| {
                                        let next_head = head_move(head, head_delta);
                                        let next_tail = tail_move(next_head, tail);

                                        tail_visits.insert(next_tail);
                                        (next_head, next_tail)
                                    }
                                )
                            }).unwrap_or_else(|| {
                                println!("invalid direction {}", direction);
                                (head, tail)
                            })
                        },
                        _ => {
                            println!("invalid distance {}", distance);
                            (head, tail)
                        }
                    }
                }
            ).unwrap_or_else(|| {
                println!("Parsing error for line {}", row);
                (head, tail)
            })
        }
    );
    println!("Tail visits {} locations", tail_visits.len())
}
