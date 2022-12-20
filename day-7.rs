use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Dir<'a> {
    // TODO name is implicit in the key into contents of the parent Dir. Probably remove it.
    name: &'a str,
    contents: HashMap<&'a str, FSObj<'a>>,
}

#[derive(Debug)]
struct File<'a> {
    // TODO maybe get rid of this name too?
    name: &'a str,
    size: u32,
}

#[derive(Debug)]
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

    let (total, accumulated) = sum_small(&root);

    println!("total: {} accumulated: {}", total, accumulated)
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
                // println!("cd /");
                if level > 0 {
                    return true; // signal that we want to unravel the recursion back to root
                }
                // If we're at level 0, we're already where we want to be; don't do anything
            }

            ["$", "cd", ".."] => {
                // println!("cd..");
                if level == 0 {
                    println!("Can't get to the parent of the root directory");
                    break; // TODO return some other value indicating error
                }
                return false;
            },

            ["$", "cd", dir_name] => {
                // println!("cd {}", dir_name);
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
                // println!("file size");
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
                break; // TODO return some other value indicating error
            }
        }
    }

    return false // end of commands
}

fn sum_small<'a>(dir : &Dir<'a>) -> (u32, u32) {
    let mut total : u32 = 0;
    let mut accum : u32 = 0;

    for (name, fs_obj) in dir.contents.iter() {
        match fs_obj {
            FSObj::Dir(next_dir) => {
                let (this_total, this_accum) = sum_small(next_dir);
                total += this_total;
                accum += this_accum;
            },
            FSObj::File(file) => {
                total += file.size;
            },
        }
    }

    return (total, accum + total)
}
