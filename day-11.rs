use std::fs;

type WorryVal = i32;
type MonkeyIndex = usize;

type WorryFunc = fn(WorryVal) -> WorryVal;
type MonkeyPassFunc = fn(WorryVal) -> MonkeyIndex;

struct Monkey {
    index : MonkeyIndex,
    items : Vec<WorryVal>,
    pass_func : MonkeyPassFunc,
    operation : WorryFunc,
}

fn read_monkeys(file_str: &str) -> Result<Vec<Monkey>, String> {
    file_str.split("\n\n").enumerate().map(|(index, monkey_string)| {
        let monkey = Monkey::read(monkey_string)?;
        assert!(monkey.index == index, "wrong monkey index on read");
        Ok(monkey)
    }).collect()
}

fn relax(worry : WorryVal) {
    worry /= 3
}

impl Monkey {
    fn read(monkey_str: &str) -> Result<Self, String> {
        let lines : Vec<&str> = monkey_str.split('\n').map(str::trim).collect();

        if lines.len() != 6 {
            return Err(["Wrong number of lines:\n\n", monkey_str].join(" "))
        }

        let index = match lines[0].split("Monkey ").collect::<Vec<&str>>()[..] {
            ["", i] => match i.split(':').collect::<Vec<&str>>()[..] {
                [i, ""] => i.parse(),
                _ => Err(["Invalid index line: ", lines[0]].join(" ")),
            },
            _ => Err(["Invalid index line: ", lines[0]].join(" ")),
        };

        let items : Option<Vec<WorryVal>> = match lines[1].split(": ").map(|a| a.split(", ")) {
            [["Starting items"], worries] => worries.map(WorryVal::parse()).collect(),
            _ => None,
        };
        if items.is_none() {
            return Err(["Invalid index line: ", lines[1]].join(" "))
        }

        let operation : Option<WorryFunc> = match lines[2].split("Operation: new = old").map(str::trim).map(|a| a.split(' ')) {
            [[""], [operator, operand]] => operand.parse().map(|num| match operator {
                "*" => Some(|old : WorryVal| old * operand),
                "+" => Some(|old : WorryVal| old + operand),
                _ => None,
            }).flatten()
        };
        if operation.is_none() {
            return Err(["Invalid operation: ", lines[2]].join(" "))
        }

        let test_val : Option<WorryVal> = match lines[3].split("Test: divisible by") {
            ["", i] => i.trim().parse(),
            _ => None,
        };
        if test_val.is_none() {
            return Err(["Invalid Test line: ", lines[3]].join(" "))
        }

        let test_success : Option<MonkeyIndex> = match lines[3].split("If true: throw to monkey") {
            ["", i] => i.trim().parse(),
            _ => None,
        };
        if test_success.is_none() {
            return Err(["Invalid Test line: ", lines[4]].join(" "))
        }

        let test_fail : Option<MonkeyIndex> = match lines[3].split("If false: throw to monkey") {
            ["", i] => i.trim().parse(),
            _ => None,
        };
        if test_fail.is_none() {
            return Err(["Invalid Test line: ", lines[5]].join(" "))
        }

        let pass_func : MonkeyPassFunc = test_val.zip(test_success).zip(test_fail).map(
            |((test_val, test_success), test_fail)|
            |worry : WorryVal| if worry % test_val == 0 {test_success} else {test_fail}
        );

        index.zip(items).zip(operation).zip(pass_func)
        .map(|(((index, items), operation), pass_func)| Monkey {
            index: index,
            items: items,
            operation: operation,
            pass_func: pass_func,
        })
        .expect("Monkey::read() - Unforseen error, something is a None that we should have accounted for before.")
    }

    fn process_monkey(monkeys : Vec<Monkey>, this_monkey : Monkey) {
        this_monkey.items.for_each(|worry|{
            worry = this_monkey.WorryFunc(worry);
            worry = relax(worry);
            if this_monkey.test(worry) {
                monkeys[this_monkey.pass_to_if_success].add_item(worry)
            } else {
                monkeys[this_monkey.pass_to_if_fail].add_item(worry)
            }
        })
    }

    fn monkey_business(monkeys : Vec<Monkey>) -> u32 {
        // TODO
    }

}

fn main() {
    let file_str = fs::read_to_string("day-10.input").expect("Failed to read file");

    let monkeys = read_monkeys(file_str);

    for _round in ..20 {
        for monkey in monkeys {
            monkey.process(&mut monkeys)
        }
    }

    println!("Monkey Business: {}", Monkey::monkey_business(monkeys));
}
