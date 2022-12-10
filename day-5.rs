use std::fs;
use std::collections::HashMap;

fn main() {
    let file_str = fs::read_to_string("day-5.input").expect("Failed to read file");

    let mut lines = file_str.trim().split("\n");

    let mut num_stacks : usize = 0;

    let initial_stacks = lines.by_ref().take_while(|line| {
        println!("{:?}", line);
        let mut labels = line.split_whitespace();
        match labels.next() {
            Some("1") => {
                num_stacks = labels.count() + 1; // since we already consumed one
                println!("{:?}", num_stacks);
                false
            },
            _ => true
        }
    });

    let mut stacks : HashMap<usize, Vec<u8>> = HashMap::new();

    initial_stacks.map(|row| {
        let mut row = row.bytes();
        for stack_num in 1.. {
            if !stacks.contains_key(&stack_num) {
                stacks.insert(stack_num, Vec::new());
            }
            match row.next().zip(row.next()).zip(row.next()) {
                Some(((_, crate_val), _)) => {
                    stacks.get_mut(&stack_num).unwrap().push(crate_val);
                    row.next(); // space in between, maybe
                },
                _ => break
            }
        }
    });

    initial_stacks.collect::<Vec<&str>>();
    ()
}
