use std::fs;

enum Instruction {
    Noop,
    AddX(i32),
}

struct SignalStateChange(i32);

#[derive(Copy, Clone, Debug)]
struct SignalState(i32);

impl Instruction {
    fn from_line(line : &str) -> Result<Instruction, String> {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", i] => i.parse()
                .map(Instruction::AddX)
                .map_err(|_| ["Failed to parse number:", i].join(" ")),
            _ => Err(["Unrecognized instruction: ", line].join(" ")),
        }
    }    

    fn to_cycle_changes(self : &Instruction) -> Vec<SignalStateChange> {
        match self {
            Instruction::Noop => vec![SignalStateChange(0)].into_iter().collect(),
            Instruction::AddX(i) => vec![SignalStateChange(0), SignalStateChange(*i)]
                .into_iter()
                .collect(),
        }
    }
}

impl SignalState {
    fn update(self : &SignalState, change: SignalStateChange) -> SignalState {
        let SignalState(x) = *self;
        let SignalStateChange(dx) = change;
        SignalState(x + dx)
    }

    fn power(self : &SignalState, cycle: i32) -> i32 {
        let SignalState(x) = *self;
        x * cycle
    }
}

fn main() {
    let file_str = fs::read_to_string("day-10.input").expect("Failed to read file");

    let instructions = file_str
        .trim()
        .split("\n")
        .map(str::trim)
        .map(Instruction::from_line);

    let initial_signal_state = SignalState(1);//, (1, Start));

    let signal_states = instructions
        .map(|i| i.expect("Failed to parse instruction"))
        .flat_map(|i| Instruction::to_cycle_changes(&i))
        .fold(
            vec![initial_signal_state],
            |states, change| {
                let &last_state = states.last().unwrap();

                states
                .into_iter()
                .chain(vec![last_state.update(change)])
                .collect()
                // unwrapping because I'm intending to start with a non-empty list and every
                // iteration I'm intending to grow it. If this fails, it's a programming error.
            }
        );

    let signal_strength: i32 = (20..221)
        .step_by(40)
        .map(|cycle| signal_states[cycle - 1].power(cycle as i32))
        .sum();
    println!("Signal strength: {}", signal_strength);

    let readout = signal_states
        .iter()
        .enumerate()
        .map(|(position, state)| {
            let SignalState(x) = state;
            if (x - (position % 40) as i32).abs() <= 1 {'#'} else {'.'}
        })
        .collect::<String>()
        .as_bytes()
        .chunks(40)
        .map(|chunk| chunk.iter().map(|&ch| ch as char).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    println!("\n\n{}", readout);
}
