use std::fs;

#[derive(PartialEq)]
enum Shape {
    Rock, Paper, Scissors
}

enum Instruction {
    Win, Lose, Draw
}

fn main () {
    let file_str = fs::read_to_string("day-2.input").expect("Failed to read file");

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
    ).fold(Some(0), |acc : Option<i32>, next| acc.zip(next).map(|(acc, next)| acc + next));

    if let Some(points) = points {
        println!("Part 1 Total points: {}", points);
    } else {
        println!("Failed to parse!")
    }

    let points = file_str.trim().split("\n").map(
        |round| {
            round.split_once(' ')
            .and_then(|(them, instruction)| {
                let instruction = match instruction {
                    "X" => Some(Instruction::Lose),
                    "Y" => Some(Instruction::Draw),
                    "Z" => Some(Instruction::Win),
                    _ => None,
                };
                let them = match them {
                    "A" => Some(Shape::Rock),
                    "B" => Some(Shape::Paper),
                    "C" => Some(Shape::Scissors),
                    _ => None,
                };
                them.zip(instruction)
            })
            .map(|(them, instruction)| {
                let me = match instruction {
                    Instruction::Win => match them {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissors,
                        Shape::Scissors => Shape::Rock,
                    },

                    Instruction::Lose => match them {
                        Shape::Rock => Shape::Scissors,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissors => Shape::Paper,
                    },

                    Instruction::Draw => match them {
                        Shape::Rock => Shape::Rock,
                        Shape::Paper => Shape::Paper,
                        Shape::Scissors => Shape::Scissors,
                    },
                };
                let shape_points = match me {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                };
                let outcome_points = match instruction {
                    Instruction::Win => 6,
                    Instruction::Lose => 0,
                    Instruction::Draw => 3,
                };
                shape_points + outcome_points
            })
        }
    ).fold(Some(0), |acc : Option<i32>, next| acc.zip(next).map(|(acc, next)| acc + next));

    if let Some(points) = points {
        println!("Part 2 Total points: {}", points);
    } else {
        println!("Failed to parse!")
    }

}
