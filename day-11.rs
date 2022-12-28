use std::fs;

type worry_val = i32;
type monkey_index = usize

type worry_func = fn(worry_val) -> worry_val;
type monkey_pass_func = fn(worry_val) -> monkey_index;

struct Monkey {
    index : monkey_index,
    items : Vec<worry_val>,
    pass_func : monkey_pass_func,
    operation : worry_func,
}

fn read_monkeys(file_str: &str) -> Result<Vec<Monkey>, Err> {
   let : monkeys = Vec<Monkey>
   file_str.split("\n\n").enumerate().map(|index, monkey_string|
        let monkey = Monkey::read()?
        assert!(monkey.index == index, "wrong monkey index on read")
   )
}

fn relax(worry : worry_val) {
    worry /= 3
}

impl Monkey {
    fn read(monkey_str: &str) -> Result<Self, String> {
        lines = monkey_str.split('\n').map(str::trim())

        if (lines.len() != 6) {
            return Err(["Wrong number of lines:\n\n", monkey_str].join(" "))
        }

        let index = match lines[0].split(' ').map(str::split(':')) {
            [["Monkey"], [i, ""]] -> i.parse(),
            _ -> None,
        }
        if (index.is_none()) {
            return Err(["Invalid index line: ", lines[0]].join(" "))
        }

        let items : Option<Vec<worry_val>> = match line[1].split(": ").map(|a| a.split(', ')) {
            [["Starting items"], worries] -> worries.map(worry_val::parse()).collect(),
            _ -> None,
        }
        if (items.is_none()) {
            return Err(["Invalid index line: ", lines[1]].join(" "))
        }

        let operation : Option<worry_func> = match line[2].split("Operation: new = old").map(str::trim).map(|a| a.split(' ')) {
            [[""], [operator, operand]] -> num->parse().map(|num| match operator {
                "*" -> Some(|old : worry_val| old * operand),
                "+" -> Some(|old : worry_val| old + operand),
                _ -> None,
            }).flatten()
        }
        if (operation.is_none()) {
            return Err(["Invalid operation: ", lines[2]].join(" "))
        }

        let test_val : Option<worry_val> = match line[3].split("Test: divisible by") {
            ["", i] -> i.trim().parse(),
            _ -> None,
        }
        if (test_val.is_none()) {
            return Err(["Invalid Test line: ", lines[3]].join(" "))
        }

        let test_success : Option<> = match line[3].split("If true: throw to monkey") {
            ["", i] -> i.trim().parse(),
            _ -> None,
        }
        if (test_success.is_none()) {
            return Err(["Invalid Test line: ", lines[4]].join(" "))
        }

        let test_success : Option<> = match line[3].split("If true: throw to monkey") {
            ["", i] -> i.trim().parse(),
            _ -> None,
        }
        if (test_success.is_none()) {
            return Err(["Invalid Test line: ", lines[5]].join(" "))
        }

        let pass_func : monkey_pass_func = test_val.zip(test_success).zip(test_fail).map(
            |((test_val, test_success), test_fail)|
            |worry : worry_val| if worry % test_val == 0 {test_success} else {test_fail}
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
            worry = this_monkey.worry_func(worry)
            worry = relax(worry)
            if this_monkey.test(worry) {
                monkeys[this_monkey.pass_to_if_success].add_item(worry)
            } else {
                monkeys[this_monkey.pass_to_if_fail].add_item(worry)
            }
        })
    }

}

fn main() {
    let file_str = fs::read_to_string("day-10.input").expect("Failed to read file");

    monkeys = read_monkeys(file_str)

    for _round in (..20) {
        for monkey in monkeys {
            monkey.process(mut &monkeys)
        }
    }

    println!("Monkey Business: {}", monkey_business(monkeys))
}
