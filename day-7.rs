use std::fs;
use std::collections::HashMap;

struct Dir<'a> {
    name: &'a str,
    contents: HashMap<&'a str, FSObj<'a>>,
}

struct File<'a> {
    name: &'a str,
    size: u32,
}

enum FSObj<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

fn main() {
    let file_str = fs::read_to_string("day-7.input").expect("Failed to read file");

    let mut root = Dir {
        name: "",
        contents: HashMap::new(),
    };

    let mut dir_stack = vec![&mut root];

    for line in file_str.trim().split('\n') {
        match line.trim().split(' ').collect::<Vec<&str>>()[..] {
            // Don't care! The only non-command output is the output of ls so we know where it came from.
            ["$", "ls"] => (),

            ["$", "cd", "/"] => dir_stack = vec![&mut root],

            ["$", "cd", ".."] => {
                dir_stack.pop();
                if dir_stack.len() == 0 {
                    println!("Can't get to the parent of root");
                    break;
                }
            },

            ["$", "cd", dir_name] => {
                let len = dir_stack.len();
                let contents = &mut dir_stack.get_mut(len - 1).unwrap().contents;
                if !contents.contains_key(&dir_name) {
                    contents.insert(dir_name, FSObj::Dir(Dir {
                        name: dir_name,
                        contents: HashMap::new(),
                    }));
                };
                let new_dir = match &contents[dir_name] {
                    FSObj::Dir(dir) => dir,
                    _ => {
                        // Could happen if it was declared as a file in a previous run
                        println!("Trying to cd into a file: {}", line);
                        break;
                    }
                };
                dir_stack.push(&mut new_dir)
            },

            // Don't care! We'll get the dir info when we cd into it and then ls
            ["dir", _] => (),

            [size, file_name] => {
                let len = dir_stack.len();
                let cur_dir = &mut dir_stack[len - 1];
                match size.to_string().parse::<u32>() {
                    Ok(size) => {
                        if !cur_dir.contents.contains_key(&file_name) {
                            cur_dir.contents.insert(file_name, FSObj::File(File {
                                name: file_name,
                                size: size,
                            }));
                        };
                    }
                    _ => {
                        println!("Invalid file size: {}", line);
                    }
                };
            }

            _ => {
                println!("Unknown command: {}", line);
                break;
            }
        }
    }
}
