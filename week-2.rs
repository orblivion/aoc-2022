use std::fs;

#[derive(PartialEq)]
enum Shape {
    Rock, Paper, Scissors
}

fn main () {
    let file_str = fs::read_to_string("week-2.input").expect("Failed to read file");

    let points = file_str.trim().split("\n").map(
        |round| {
            round.split_once(' ')
            .and_then(|(them, me)| {
                let me = match me {
                    "X" => Some(Shape::Rock),
                    "Y" => Some(Shape::Paper),
                    "Z" => Some(Shape::Scissors),
                    _ => None,
                };
                let them = match them {
                    "A" => Some(Shape::Rock),
                    "B" => Some(Shape::Paper),
                    "C" => Some(Shape::Scissors),
                    _ => None,
                };
                them.zip(me)
            })
            .map(|(them, me)| {
                let shape_points = match me {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                };
                let outcome_points = match (me, them) {
                    // Loss conditions
                    (Shape::Rock, Shape::Paper) => 0,
                    (Shape::Paper, Shape::Scissors) => 0,
                    (Shape::Scissors, Shape::Rock) => 0,

                    (me, them) => match me == them {
                        // Draw conditions
                        true => 3,
                        
                        // Win conditions (by elimination)
                        false => 6
                    },
                };
                shape_points + outcome_points
            })
        }
    );

    // map sum whatever
    
    /*
    if let Some(points) = points {
    } else {
        println!("Failed to parse round: {}", points);
    }
    */
}
