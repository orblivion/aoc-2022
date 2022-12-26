use std::fs;
use std::collections::HashSet;

fn head_move((x, y), (dx, dy)) -> (u32, u32) {
    return (x + dx, y + dy)
}

fn tail_move((head_x, head_y), (tail_x, tail_y)) -> (u32, u32) {
    return (
        max(min(head_x, tail_x + 1), tail_x - 1),
        max(min(head_y, tail_y + 1), tail_y - 1),
    )
}

fn main() {
    let file_str = fs::read_to_string("day-9.input").expect("Failed to read file");

    let mut tail_visits = HashSet<(u32, u32)> = HashSet::new()

    file_str.trim().split("\n").fold(
        ((0, 0), (0, 0)),
        |((head, tail) row)| row.split_once(' ').and_then(
            |(dir, distance)| {
                distance.to_string().parse::<u32>() {
                    Ok(distance) => {
                        let head_delta = match dir {
                            'D': (1, 0),
                            'U': (-1, 0),
                            'L': (0, -1),
                            'R': (0, 1),
                        }
                        let next_head = head_move(head, head_delta);
                        let next_tail = tail_move(next_head, tail);

                        tail_visits.insert((x as u32, y as u32));
                        return next_head, next_tail
                    }
                    _ => {
                        println!("invalid distance")
                    }
                }
            }
        )
    )
    println!("Tail visits {} locations", tail_visit.len())
}
