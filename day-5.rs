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
            if !stacks.contains_key(&stack_num) {
                stacks.insert(stack_num, Vec::new());
            }
            match row.next()
                  .zip(row.next())
                  .zip(row.next())
                  .map(|((a, b), c)| (a,b,c)) {
                Some((' ', ' ', ' ')) => {
                    row.next(); // space in between ']' and '[' if not at the end of the row
                },
                Some(('[', crate_val, ']')) => {
                    stacks.get_mut(&stack_num).unwrap().push(crate_val);
                    row.next(); // space in between ']' and '[' if not at the end of the row
                },
                _ => break
            }
        }
    };

    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    let instructions = reader;

    for row in instructions {
        let words : Vec<&str> = row.split(" ").collect();

        match words[..] {
            ["move", amount, "from", from, "to", to] => {
                let amount : i32 = amount.to_string().parse().unwrap();
                let from : usize = from.to_string().parse().unwrap();
                let to : usize = to.to_string().parse().unwrap();

                for _ in 0..amount {
                    let crate_val = stacks.get_mut(&from).map(|val| val.pop());
                    match crate_val {
                        Some(Some(crate_val)) => {stacks.get_mut(&to).map(|val| val.push(crate_val));},
                        Some(None) => println!("Stack {} ran out of crates!", from),
                        None => println!("Some other weird stuff happened!"),
                    };
                }
            }
            _ => ()
        };
    }

    for stack in stacks.iter() {
        println!("{:?}", stack);
    }
}
