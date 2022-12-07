use std::fs;
use std::collections::HashSet;

fn main () {
    let file_str = fs::read_to_string("week-3.input").expect("Failed to read file");

    let points : i32 = file_str.trim().split("\n").map(
        |rucksack| {
            let rucksack : Vec<u8> = rucksack.bytes().collect();
            let compartment_len = rucksack.len() / 2;
            let compartment_1 : HashSet<&u8> = rucksack[0..compartment_len - 1].iter().collect();
            let compartment_2 : HashSet<&u8> = rucksack[compartment_len..rucksack.len() - 1].iter().collect();

            let &&b = compartment_1.intersection(&compartment_2).next().unwrap();
            b
        }
    ).map(
        |letter| -> i32 {
            if letter >= b'a' && letter <= b'z' {
                return (letter - b'a' + 1).into()
            }
            if letter >= b'A' && letter <= b'Z' {
                return (letter - b'A' + 27).into()
            }
            return 0
        }
    ).sum();

    println!("Points: {}", points);
}
