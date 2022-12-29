use std::fs;

type WorryVal = i32;
type MonkeyIndex = usize;
type MonkeyBusiness = u32;

enum Operator {Mult, Add}

struct Monkey {
    index : MonkeyIndex,
    items : Vec<WorryVal>,
    pass_func : (WorryVal, MonkeyIndex, MonkeyIndex),
    operation : (Operator, WorryVal),

    pass_count : MonkeyBusiness,
}

fn read_monkeys(file_str: &str) -> Result<Vec<Monkey>, String> {
    file_str.split("\n\n").enumerate().map(|(index, monkey_string)| {
        let monkey = Monkey::read(monkey_string)?;
        assert!(monkey.index == index, "wrong monkey index on read");
        Ok(monkey)
    }).collect()
}

fn relax(worry : WorryVal) -> WorryVal {
    worry / 3
}

impl Monkey {
    fn read(monkey_str: &str) -> Result<Self, String> {
        let lines : Vec<&str> = monkey_str.split('\n').map(str::trim).collect();

        if lines.len() != 6 {
            return Err(["Wrong number of lines:\n\n", monkey_str].join(" "))
        }

        let index = match lines[0].split("Monkey ").collect::<Vec<&str>>()[..] {
            ["", i] => match i.split(':').collect::<Vec<&str>>()[..] {
                [i, ""] => i.parse::<MonkeyIndex>().map_err(|x| x.to_string()),
                _ => Err(["Invalid index line: ", lines[0]].join(" ")),
            },
            _ => Err(["Invalid index line: ", lines[0]].join(" ")),
        }?;

        let items = match lines[1].split("Starting items: ").collect::<Vec<&str>>()[..] {
            ["", worries] => worries
                .split(", ")
                .map(|x| x.parse::<WorryVal>()
                     .map_err(|x| x.to_string()))
                .collect(),
            _ => Err(["Invalid items line: ", lines[1]].join(" ")),
        }?;

        let operation = match lines[2].split("Operation: new = old").map(str::trim).collect::<Vec<&str>>()[..] {
            ["", operation] => {
                match operation.split(" ").collect::<Vec<&str>>()[..] {
                    [operator, operand] => operand.parse::<WorryVal>()
                        .map_err(|x| x.to_string())
                        .map(|operand| match operator {
                            "+" => Ok((Operator::Add, operand)),
                            "*" => Ok((Operator::Mult, operand)),
                            _ => Err(["Invalid operation: ", lines[2]].join(" ")),
                        }),
                    _ => Err(["Invalid operation: ", lines[2]].join(" ")),
                }
            }
            _ => Err(["Invalid operation: ", lines[2]].join(" ")),
        }??; // No Result flattening available

        let pass_divisible_by = match lines[3].split("Test: divisible by").collect::<Vec<&str>>()[..] {
            ["", i] => i.trim().parse::<WorryVal>().map_err(|x| x.to_string()),
            _ => Err(["Invalid Test line: ", lines[3]].join(" ")),
        }?;

        let pass_success = match lines[4].split("If true: throw to monkey").collect::<Vec<&str>>()[..] {
            ["", i] => i.trim().parse::<MonkeyIndex>().map_err(|x| x.to_string()),
            _ => Err(["Invalid Test line: ", lines[4]].join(" ")),
        }?;

        let pass_fail = match lines[5].split("If false: throw to monkey").collect::<Vec<&str>>()[..] {
            ["", i] => i.trim().parse::<MonkeyIndex>().map_err(|x| x.to_string()),
            _ => Err(["Invalid Test line: ", lines[5]].join(" ")),
        }?;

        Ok(Monkey {
            index: index,
            items: items,
            operation: operation,
            pass_func: (pass_divisible_by, pass_success, pass_fail),
            pass_count: 0,
        })
    }

    fn process(monkeys : &mut Vec<Monkey>, processing_index : MonkeyIndex) {
        let processing_monkey = &mut monkeys[processing_index];
        let mut pass_count = processing_monkey.pass_count;
        processing_monkey.items.iter().for_each(|worry|{
            let worry = match processing_monkey.operation {
                (Operator::Mult, operand) => worry * operand,
                (Operator::Add, operand) => worry * operand,
            };
            let worry = relax(worry);
            let (divisible_by, if_success, if_fail) = processing_monkey.pass_func;
            if worry % divisible_by == 0 {
                monkeys[if_success].items.push(worry)
            } else {
                monkeys[if_fail].items.push(worry)
            }
            pass_count += 1;
        });
        processing_monkey.pass_count = pass_count;
        processing_monkey.items = Vec::new()
    }

    fn monkey_business(monkeys : Vec<Monkey>) -> MonkeyBusiness {
        let mut top = monkeys
            .iter()
            .map(|monkey| monkey.pass_count)
            .collect::<Vec<MonkeyBusiness>>();
        top.sort();
        top[monkeys.len() - 1] * top[monkeys.len() - 2]
    }

}

fn main() {
    let file_str = fs::read_to_string("day-10.input").expect("Failed to read file");

    read_monkeys(&file_str[..])
    .map(|mut monkeys| {
        for _round in 0..20 {
            for index in 0..monkeys.len() {
                Monkey::process(&mut monkeys, index)
            }
        }

        println!("Monkey Business: {}", Monkey::monkey_business(monkeys));
    })
    .map_err(|E| println!("Error: {}", E));
}
