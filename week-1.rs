use std::fs;

fn main () {
    // 
    let file_str = fs::read_to_string("week-1.input").expect("Failed to read file");

    let elf_str_sections : Vec<&str> = file_str.trim().split("\n\n").collect();

    let elf_calories : Option<i32> = elf_str_sections.iter().map(
        |&a| {
            let x : Vec<&str> = a.trim().split("\n").collect();
            x.iter().map(
                |&b| {
                    b.to_string().parse::<i32>().unwrap()
                }
            ).sum()
        }
    ).max();

    
    println!("{}", elf_calories.unwrap());
}
