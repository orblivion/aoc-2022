use std::fs;
use std::collections::HashMap;

struct Dir<'a> {
    // TODO name is implicit in the key into contents of the parent Dir. Probably remove it.
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

    let mut commands = file_str.trim().split('\n');

    build(&mut root, &mut commands, 0);
}

// Returns whether we are doing `cd /`
fn build<'a>(dir : &mut Dir<'a>, commands : &mut impl Iterator<Item = &'a str>, level : usize) -> bool {
    loop {
        let line = if let Some(line) = commands.next() {
            line
        } else {
            break
        };

        match line.trim().split(' ').collect::<Vec<&str>>()[..] {
            // Don't care! The only non-command output is the output of ls so we know where it came from.
            ["$", "ls"] => (),

            ["$", "cd", "/"] => {
                return true; // signal that we want to unravel the recursion back to root
            }

            ["$", "cd", ".."] => {
                if level == 0 {
                    println!("Can't get to the parent of the root directory");
                    break;
                }
                return false; // signal that we only want to go up one directory
            },

            ["$", "cd", dir_name] => {
                // If this is our first visit into this directory, create it.
                if !dir.contents.contains_key(&dir_name) {
                    dir.contents.insert(dir_name, FSObj::Dir(Dir {
                        name: dir_name,
                        contents: HashMap::new(),
                    }));
                };

                // Get the directory, whether or not we just made it
                let mut next_dir = match dir.contents.get_mut(dir_name).unwrap() {
                    FSObj::Dir(dir) => dir,
                    _ => {
                        // Could happen if it was declared as a file in a previous run
                        println!("Trying to cd into a file: {}", line);
                        return false; // TODO return some other value indicating error
                    }
                };
                if build(&mut next_dir, commands, level + 1) && level > 0 {
                    return true // we got the signal to unravel back to root. return and pass the signal on.
                }
            },

            // Don't care! We'll get the dir info when we cd into it and then ls
            ["dir", _] => (),

            [size, file_name] => {
                match size.to_string().parse::<u32>() {
                    Ok(size) => {
                        if !dir.contents.contains_key(&file_name) {
                            dir.contents.insert(file_name, FSObj::File(File {
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

    return false // end of commands
}
