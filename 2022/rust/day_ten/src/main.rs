use std::fs;
use std::str::FromStr;

enum Command {
    NoOp,
    AddX(i32),
}

impl Command {
    fn from_str(line: &str) -> Self {
        let command_parts: Vec<&str> = line.split_whitespace().collect();

        match command_parts[..] {
            [_] => Self::NoOp,
            [_, arg] => Self::AddX(i32::from_str(arg).unwrap()),
            _ => panic!("Unexpected input: {}", line),
        }
    }
}

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn is_sample_point(cycle: i32) -> bool {
    (cycle - 20) % 40 == 0
}

fn get_next_accumulated_value(cycle: i32, register: i32, current_value: i32) -> i32 {
    if is_sample_point(cycle) {
        current_value + (cycle * register)
    } else {
        current_value
    }
}

fn part_one() {
    let input = get_data();
    let commands = input.lines().map(|line| Command::from_str(line));
    let mut cycle = 1;
    let mut register = 1;
    let mut accumulator = 0;

    for cmd in commands {
        match cmd {
            Command::NoOp => {
                cycle += 1;
                accumulator = get_next_accumulated_value(cycle, register, accumulator);
            }
            Command::AddX(arg) => {
                cycle += 1;
                accumulator = get_next_accumulated_value(cycle, register, accumulator);
                cycle += 1;
                register += arg;
                accumulator = get_next_accumulated_value(cycle, register, accumulator);
            }
        }
    }

    println!("accumulated value = {}", accumulator);
}

fn part_two() {
    let input = get_data();
    let mut commands = input.lines().map(|line| Command::from_str(line)).peekable();
    let mut current_pixel: i32 = 0;
    let mut register: i32 = 1;
    let mut screen = String::from("");
    let mut addx_in_progress = false;

    loop {
        if current_pixel % 40 == 0 {
            screen.push('\n');
        }
        if ((current_pixel % 40) - register).abs() < 2 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if addx_in_progress {
            let next_command = commands.next();

            match next_command {
                Some(Command::AddX(arg)) => {
                    register += arg;
                    addx_in_progress = false;
                }
                _ => {
                    panic!("Unexpected command.")
                }
            }
        } else {
            match commands.peek() {
                Some(Command::AddX(_)) => {
                    addx_in_progress = true;
                }
                Some(Command::NoOp) => {
                    commands.next();
                }
                None => {
                    break;
                }
            }
        }

        current_pixel += 1;
    }

    println!("{}", screen);
}

fn main() {
    part_two();
}
