use std::fs;

enum Instruction {
    Noop,
    AddX(i32),
}

enum CyclePart {Start, During, After}

struct SignalState {
    x : i32,
    cycle: (u32, CyclePart),
}

impl Instruction {
    fn from_line(s : &str) -> Instruction {
        Instruction::Noop
    }    
}

fn main() {
    let file_str = fs::read_to_string("day-10.input").expect("Failed to read file");

    file_str.trim().split("\n").map(Instruction::from_line);
}
