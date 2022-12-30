use std::fs;
use std::collections::HashMap;

type WorryVal = u64;
type MonkeyIndex = usize;
type MonkeyBusiness = u64;

#[derive(Clone, Debug)]
enum Operator {Mult, Add}

#[derive(Clone, Debug)]
enum Operand {Old, Number(WorryVal)}

#[derive(Clone, Debug)]
struct Monkey {
    index : MonkeyIndex,
    items : Vec<WorryVal>,
    pass_func : (WorryVal, MonkeyIndex, MonkeyIndex),
    operation : (Operand, Operator, Operand),

    pass_count : MonkeyBusiness,
}

fn factor_number(number: WorryVal) -> HashMap::<WorryVal, u32> {
    let mut number = number;
    let mut factors : HashMap<WorryVal, u32>= HashMap::new();

    'outer: for factor in 2.. {
        // Don't care if it's prime. For instance for something divisible by 4, it'll hit 2 twice,
        // taking care of the 4. So we'll only be saving primes.
        loop {
            if number == 1 {
                break 'outer
            }
            if number % factor == 0 {
                number /= factor;
                if factors.contains_key(&factor) {
                    factors.insert(factor, factors[&factor] + 1);
                } else {
                    factors.insert(factor, 1);
                }
            } else {
                break;
            }
        }
    }

    return factors
}

fn merge_factors(factors : &mut HashMap::<WorryVal, u32>, new_factors : HashMap::<WorryVal, u32>) {
    for (factor, count) in new_factors {
        if !factors.contains_key(&factor) || count > factors[&factor] {
            factors.insert(factor, count);
        }
    }
}

// Factor every number in the system and multiply it together
fn get_relax_factor(monkeys : &Vec<Monkey>) -> WorryVal {
    let mut factors : HashMap<WorryVal, u32>= HashMap::new();

    for monkey in monkeys {
        let (criterion, _, _) = monkey.pass_func;
        let new_factors = factor_number(criterion);
        merge_factors(&mut factors, new_factors);
    }

    let mut relax_factor = 1;

    for (factor, count) in factors {
        for _ in 0..count {
            relax_factor *= factor;
        }
    }

    relax_factor
}

fn read_monkeys(file_str: &str) -> Result<Vec<Monkey>, String> {
    file_str.trim().split("\n\n").enumerate().map(|(index, monkey_string)| {
        let monkey = Monkey::read(monkey_string)?;
        assert!(monkey.index == index, "wrong monkey index on read");
        Ok(monkey)
    }).collect()
}

fn relax_always(worry : WorryVal, relax_amount : WorryVal) -> WorryVal {
    worry / relax_amount
}

fn relax_on_divisible(worry : WorryVal, relax_amount : WorryVal) -> WorryVal {
    if worry % relax_amount == 0 {
        worry / relax_amount
    } else {
        worry
    }
}

fn parse_operand(s : &str) -> Result<Operand, String> {
    match s {
        "old" => Ok(Operand::Old),
        i => i.parse::<WorryVal>()
            .map(|i| Operand::Number(i))
            .map_err(|e| format!("Invalid operand: {} - {}", s, e)),
    }
}

fn parse_operator(s : &str) -> Result<Operator, String> {
    match s {
        "+" => Ok(Operator::Add),
        "*" => Ok(Operator::Mult),
        _ => Err(format!("Invalid operator: {}", s))
    }
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
                _ => Err("".to_string()),
            },
            _ => Err("".to_string())
        }
        .map_err(|e| format!("Invalid index line: {} - {}", lines[0], e))?;

        let items = match lines[1].split("Starting items: ").collect::<Vec<&str>>()[..] {
            ["", worries] => worries
                .split(", ")
                .map(|x| x.parse::<WorryVal>()
                     .map_err(|x| x.to_string()))
                .collect(),
            _ => Err(["Invalid items line: ", lines[1]].join(" ")),
        }?;

        let operation = match lines[2].split("Operation: new =").map(str::trim).collect::<Vec<&str>>()[..] {
            ["", operation] => {
                match operation.split(" ").collect::<Vec<&str>>()[..] {
                    [left, operator, right] => {
                        (|| {
                            let left = parse_operand(left)?;
                            let operator = parse_operator(operator)?;
                            let right = parse_operand(right)?;
                            Ok((left, operator, right))
                        })()
                        .map_err(|e: String| format!("Invalid operation: {} - {}", lines[2], e))
                    }
                    _ => Err(format!("Invalid operation: {}", lines[2]))
                }
            }
            _ => Err(format!("Invalid operation: {}", lines[2]))
        }?;

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

    fn process(monkeys : &mut Vec<Monkey>, processing_index : MonkeyIndex, relax_amount : WorryVal, should_relax_on_divisible : bool) {
        let processing_monkey = &mut monkeys[processing_index];

        let changes = processing_monkey.items.iter()
            .map(|worry|{
                let (left, operator, right) = processing_monkey.operation.clone();
                let left = match left {
                    Operand::Old => *worry,
                    Operand::Number(i) => i,
                };
                let right = match right {
                    Operand::Old => *worry,
                    Operand::Number(i) => i,
                };
                // println!("{:?} {:?} {:?}", left, operator, right);
                let worry = match operator {
                    Operator::Mult => left * right,
                    Operator::Add => left + right,
                };
                // println!("{} {}", worry, relax_amount);
                let worry = if should_relax_on_divisible {
                    relax_on_divisible(worry, relax_amount)
                } else {
                    relax_always(worry, relax_amount)
                };
                let (divisible_by, if_success, if_fail) = processing_monkey.pass_func;
                if worry % divisible_by == 0 {
                    return (if_success, worry)
                } else {
                    return (if_fail, worry)
                }
            }).collect::<Vec<(MonkeyIndex, WorryVal)>>();

        changes
            .into_iter()
            .for_each(|(to_index, worry)| monkeys[to_index].items.push(worry));

        let processing_monkey = &mut monkeys[processing_index];

        processing_monkey.pass_count += processing_monkey.items.len() as MonkeyBusiness;
        processing_monkey.items = Vec::new()
    }

    fn monkey_business(monkeys : &Vec<Monkey>) -> MonkeyBusiness {
        let mut top = monkeys
            .iter()
            .map(|monkey| monkey.pass_count)
            .collect::<Vec<MonkeyBusiness>>();
        top.sort();
        top[monkeys.len() - 1] * top[monkeys.len() - 2]
    }

}

fn main() {
    let file_str = fs::read_to_string("day-11.input").expect("Failed to read file");

    let monkey_businesses = read_monkeys(&file_str[..])
    .map(|monkeys| {
        let mut relax_20_monkeys = monkeys.clone();
        for _round in 0..20 {
            for index in 0..relax_20_monkeys.len() {
                Monkey::process(&mut relax_20_monkeys, index, 20, false)
            }
        }
        let monkey_business_20 = Monkey::monkey_business(&relax_20_monkeys);

        let mut relax_factors_monkeys = monkeys.clone();
        let relax_factor = get_relax_factor(&relax_factors_monkeys);
        for round in 0..10000 {
            // println!("{}", round);
            for index in 0..relax_factors_monkeys.len() {
                Monkey::process(&mut relax_factors_monkeys, index, relax_factor, true);
            }
        }
        let monkey_business_10000 = Monkey::monkey_business(&relax_factors_monkeys);

        (monkey_business_20, monkey_business_10000)
    });

    match monkey_businesses {
        Ok((monkey_business_20, monkey_business_10000)) => {
            println!("Monkey Business at 20: {}", monkey_business_20);
            println!("Monkey Business at 10000: {}", monkey_business_10000);
        },
        Err(e) => println!("Error: {}", e),
    };
}
