use std::fs;

fn main () {
    let file_str = fs::read_to_string("week-1.input").expect("Failed to read file");

    let elf_str_sections : Vec<&str> = file_str.trim().split("\n\n").collect();

    let mut elf_calories : Vec<i32> = elf_str_sections.iter().map(
        |&elf_str_section| {
            let elf_strs : Vec<&str> = elf_str_section.trim().split("\n").collect();
            elf_strs.iter().map(
                |&calorie_str| {
                    calorie_str.to_string().parse::<i32>().unwrap()
                }
            ).sum()
        }
    ).collect();

    elf_calories.sort();

    let elf_calories_rev : Vec<i32> = elf_calories.iter().rev().map(|&x| x).collect();
    
    println!("Top elf: {}", elf_calories_rev[0]);
    println!("Top 3 elves: {}", elf_calories_rev[0] + elf_calories_rev[1] + elf_calories_rev[2]);
}
