use num::{BigUint, Zero};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::FromStr;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone)]
enum Operand {
    Old,
    New(BigUint),
}

#[derive(Clone)]
enum InspectionResult {
    Pass(BigUint),
    Fail(BigUint),
}

struct Monkey {
    operation: Operation,
    operand: Operand,
    divisor: u32,
    success_target: usize,
    fail_target: usize,
    activity_count: u64,
}

impl Monkey {
    fn new(
        operation: Operation,
        operand: Operand,
        divisor: u32,
        success_target: usize,
        fail_target: usize,
    ) -> Self {
        Self {
            operation,
            operand,
            divisor,
            success_target,
            fail_target,
            activity_count: 0u64,
        }
    }

    fn success_target(&self) -> usize {
        self.success_target
    }

    fn fail_target(&self) -> usize {
        self.fail_target
    }

    fn activity_count(&self) -> u64 {
        self.activity_count
    }

    fn inspect_item(&mut self, item: BigUint) -> InspectionResult {
        println!("  Monkey inspects an item with a worry level of {}.", item);
        self.activity_count += 1;

        let adjusted_item: BigUint;

        let op = match self.operand.clone() {
            Operand::Old => item.clone(),
            Operand::New(i) => i.clone(),
        };

        match self.operation {
            Operation::Add => {
                println!("    Worry level increases by {} to {}.", &op, &item + &op);
                adjusted_item = item + op;
            }
            Operation::Multiply => {
                println!(
                    "    Worry level is multiplied by {} to {}.",
                    &op,
                    &item * &op
                );
                adjusted_item = item * op;
            }
        }

        // println!(
        //    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
        //    &adjusted_item / 3u32
        //);
        //let final_adjusted_item = adjusted_item / 3u32;
        let final_adjusted_item = adjusted_item;

        match (&final_adjusted_item % self.divisor) == Zero::zero() {
            true => {
                println!("    Current worry level is divisible by {}.", self.divisor);
                InspectionResult::Pass(final_adjusted_item)
            }
            false => {
                println!(
                    "    Current worry level is not divisible by {}.",
                    self.divisor
                );
                InspectionResult::Fail(final_adjusted_item)
            }
        }
    }
}

fn part_one() {
    let input = get_data();
    let lines = input.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut item_map: HashMap<usize, VecDeque<BigUint>> = HashMap::new();
    let mut current_operation: Option<Operation> = None;
    let mut current_operand: Option<Operand> = None;
    let mut current_divisor: Option<u32> = None;
    let mut current_success_target: Option<usize> = None;
    let mut current_fail_target: Option<usize> = None;
    let mut current_monkey_index: Option<usize> = None;
    let mut lcm = 1u32;

    for line in lines {
        match &line
            .split_whitespace()
            .filter(|p| !p.is_empty())
            .collect::<Vec<&str>>()[..]
        {
            ["Monkey", arg] => {
                if let Ok(i) = usize::from_str(arg.trim_end_matches(":")) {
                    current_monkey_index = Some(i);
                    item_map.insert(i, VecDeque::new());
                }
            }
            ["Starting", "items:"] => {}
            ["Starting", "items:", item_list @ ..] => {
                for item in item_list.iter().map(|s| {
                    if let Ok(i) = BigUint::from_str(s.trim_end_matches(',')) {
                        i
                    } else {
                        panic!("")
                    }
                }) {
                    if let Some(monkey_index) = current_monkey_index {
                        item_map.get_mut(&monkey_index).unwrap().push_back(item);
                    }
                }
            }
            ["Operation:", "new", "=", "old", "+", "old"] => {
                current_operation = Some(Operation::Add);
                current_operand = Some(Operand::Old);
            }
            ["Operation:", "new", "=", "old", "*", "old"] => {
                current_operation = Some(Operation::Multiply);
                current_operand = Some(Operand::Old);
            }
            ["Operation:", "new", "=", "old", "+", arg] => {
                current_operation = Some(Operation::Add);
                current_operand = match BigUint::from_str(arg) {
                    Ok(val) => Some(Operand::New(val)),
                    Err(err) => panic!("{:?}", err),
                };
            }
            ["Operation:", "new", "=", "old", "*", arg] => {
                current_operation = Some(Operation::Multiply);
                current_operand = match BigUint::from_str(arg) {
                    Ok(val) => Some(Operand::New(val)),
                    Err(err) => panic!("{:?}", err),
                };
            }
            ["Test:", "divisible", "by", arg] => {
                if let Ok(i) = u32::from_str(arg) {
                    current_divisor = Some(i);
                } else {
                    panic!("{} is not valid for divisor", arg);
                }
            }
            ["If", "true:", "throw", "to", "monkey", arg] => {
                if let Ok(i) = usize::from_str(arg) {
                    current_success_target = Some(i);
                }
            }
            ["If", "false:", "throw", "to", "monkey", arg] => {
                if let Ok(i) = usize::from_str(arg) {
                    current_fail_target = Some(i);
                }
            }
            [] => {
                monkeys.push(Monkey::new(
                    current_operation.unwrap(),
                    current_operand.unwrap(),
                    current_divisor.unwrap(),
                    current_success_target.unwrap(),
                    current_fail_target.unwrap(),
                ));

                lcm *= current_divisor.unwrap();

                current_operation = None;
                current_operand = None;
                current_divisor = None;
                current_success_target = None;
                current_fail_target = None;
            }
            _ => {
                panic!("Cannot parse input line: {}", line);
            }
        }
    }

    monkeys.push(Monkey::new(
        current_operation.unwrap(),
        current_operand.unwrap(),
        current_divisor.unwrap(),
        current_success_target.unwrap(),
        current_fail_target.unwrap(),
    ));

    lcm *= current_divisor.unwrap();

    for round in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            println!("Monkey {}:", monkey_index);
            loop {
                if let Some(item) = item_map.get_mut(&monkey_index).unwrap().pop_front() {
                    match monkeys[monkey_index].inspect_item(item) {
                        InspectionResult::Pass(inspected_item) => {
                            let target = monkeys[monkey_index].success_target();
                            println!(
                                "    Item with worry level {} is thrown to monkey {}.",
                                inspected_item, target
                            );
                            item_map
                                .get_mut(&target)
                                .unwrap()
                                .push_back(inspected_item % lcm);
                        }
                        InspectionResult::Fail(inspected_item) => {
                            let target = monkeys[monkey_index].fail_target();
                            println!(
                                "    Item with worry level {} is thrown to monkey {}.",
                                inspected_item, target
                            );
                            item_map
                                .get_mut(&target)
                                .unwrap()
                                .push_back(inspected_item % lcm);
                        }
                    }
                } else {
                    break;
                }
            }
        }
        println!("after round {}", round + 1);
        if round == 0 || round == 19 || round == 999 || round == 1999 {
            println!("{}", monkeys[0].activity_count());
            println!("{}", monkeys[1].activity_count());
            println!("{}", monkeys[2].activity_count());
        }
    }

    monkeys.sort_by_key(|m| m.activity_count());
    let monkey_business = monkeys
        .iter()
        .rev()
        .take(2)
        .fold(1u64, |acc, m| m.activity_count() * acc);
    println!("Monkey business: {}", monkey_business);
}

fn main() {
    part_one();
}
