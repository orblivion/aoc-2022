use std::fs;
use std::collections::HashSet;

fn sum_letters(letters: &mut dyn Iterator<Item = u8>) -> i32 {
    letters.map(
        |letter| -> i32 {
            if letter >= b'a' && letter <= b'z' {
                return (letter - b'a' + 1).into()
            }
            if letter >= b'A' && letter <= b'Z' {
                return (letter - b'A' + 27).into()
            }
            return 0
        }
    ).sum()
}

fn main () {
    let file_str = fs::read_to_string("day-3.input").expect("Failed to read file");

    let mut letters = file_str.trim().split("\n").map(
        |rucksack| {
            let rucksack : Vec<u8> = rucksack.bytes().collect();
            let compartment_len = rucksack.len() / 2;
            let compartment_1 : HashSet<u8> = rucksack[0..compartment_len].iter().map(|&x| x).collect();
            let compartment_2 : HashSet<u8> = rucksack[compartment_len..rucksack.len()].iter().map(|&x| x).collect();

            compartment_1.intersection(&compartment_2).map(|&x| x).next().unwrap()
        }
    );

    let points = sum_letters(&mut letters);

    println!("Points: {}", points);

    let file_str = fs::read_to_string("day-3.input2").expect("Failed to read file");

    let mut lines = file_str.trim().split("\n");
    let mut letters = std::iter::from_fn(move || {
        // 3 lines, get the group
        lines.next()
        .zip(lines.next())
        .zip(lines.next())
        // clean up the tuple structure
        .map(|((x, y), z)|(x, y, z))
    })
    .map(|(elf1, elf2, elf3)| {
        let elf1 : HashSet<u8> = elf1.bytes().collect();
        let elf2 : HashSet<u8> = elf2.bytes().collect();
        let elf3 : HashSet<u8> = elf3.bytes().collect();
        elf1.intersection(&elf2)
            .map(|&x| x)
            .collect::<HashSet<u8>>()
            .intersection(&elf3)
            .map(|&x| x)
            .next()
            .unwrap()
    });

    let points = sum_letters(&mut letters);

    println!("Points: {}", points);
}
