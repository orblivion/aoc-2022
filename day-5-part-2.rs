use std::fs;
use std::collections::HashMap;

fn main() {
    let file_str = fs::read_to_string("day-5.input").expect("Failed to read file");

    let mut lines = file_str.trim_end().split("\n");

    let reader = lines.by_ref();

    let initial_stacks = reader.take_while(|line| {
        // println!("{:?}", line);
        let mut labels = line.split_whitespace();
        match labels.next() {
            Some("1") => {
                false
            },
            _ => true
        }
    });

    let mut stacks : HashMap<usize, Vec<char>> = HashMap::new();

    for row in initial_stacks {
        let mut row = row.chars();
        // println!("{:?}", row);
        for stack_num in 1.. {
            match row.next()
                  .zip(row.next())
                  .zip(row.next())
                  .map(|((a, b), c)| (a,b,c)) {
                Some((' ', ' ', ' ')) => {
                    row.next(); // space in between ']' and '[' if not at the end of the row
                },
                Some(('[', crate_val, ']')) => {
                    if !stacks.contains_key(&stack_num) {
                        stacks.insert(stack_num, Vec::new());
                    }
                    stacks.get_mut(&stack_num).unwrap().push(crate_val);
                    row.next(); // space in between ']' and '[' if not at the end of the row
                },
                _ => break
            }
        }
    };

    for stack_num in 1.. {
        if !stacks.contains_key(&stack_num) {
            break;
        }
        println!("{} {:?} (before)", stack_num, stacks[&stack_num]);
        stacks.insert(stack_num,
            stacks[&stack_num].iter().map(|&x| x).rev().collect()
        );
        println!("{} {:?} (before - rev)", stack_num, stacks[&stack_num]);
    }

    let instructions = reader;

    for row in instructions {
        let words : Vec<&str> = row.split(" ").collect();

        match words[..] {
            ["move", amount, "from", from, "to", to] => {

                println!("{:?}", ["move", amount, "from", from, "to", to]);

                let amount : i32 = amount.to_string().parse().unwrap();
                let from : usize = from.to_string().parse().unwrap();
                let to : usize = to.to_string().parse().unwrap();

                let mut tmp : Vec<char> = Vec::new();

                for _ in 0..amount {
                    println!("{} {:?} (during)", from, stacks.get_mut(&from));
                    let crate_val = stacks.get_mut(&from).map(|val| val.pop());
                    match crate_val {
                        Some(Some(crate_val)) => {tmp.push(crate_val)},
                        Some(None) => println!("Stack {} ran out of crates!", from),
                        None => println!("Some other weird stuff happened!"),
                    };
                }
                println!("{} {:?} (during)", from, stacks.get_mut(&from));

                for crate_val in tmp.iter().rev() {
                    println!("{} {:?} (during)", to, stacks.get_mut(&to));
                    stacks.get_mut(&to).map(|val| val.push(*crate_val));
                }
                println!("{} {:?} (during)", to, stacks.get_mut(&to));
            }
            _ => ()
        };
    }

    for stack in stacks.iter() {
        println!("{:?} (after)", stack);
    }

    let mut result : Vec<char> = Vec::new();

    for stack_num in 1..10 {
        match stacks.get_mut(&stack_num).map(|val| val.pop()) {
            Some(Some(l)) => {
                result.push(l);
                println!("{}, {:?} {:?}", stack_num, l, stacks.get_mut(&stack_num));
            }
            None => break,
            _ => println!("Some weird stuff happened!"),
        }
    }

    println!("{}", result.iter().collect::<String>())
}
