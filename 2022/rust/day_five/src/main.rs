use std::fs;
use std::str::FromStr;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn part_one() {
    let input = get_data();

    let mut stacks: Vec<Vec<char>> = vec![
        vec!['J', 'H', 'G', 'M', 'Z', 'N', 'T', 'F'],
        vec!['V', 'W', 'J'],
        vec!['G', 'V', 'L', 'J', 'B', 'T', 'H'],
        vec!['B', 'P', 'J', 'N', 'C', 'D', 'V', 'L'],
        vec!['F', 'W', 'S', 'M', 'P', 'R', 'G'],
        vec!['G', 'H', 'C', 'F', 'B', 'N', 'V', 'M'],
        vec!['D', 'H', 'G', 'M', 'R'],
        vec!['H', 'N', 'M', 'V', 'Z', 'D'],
        vec!['G', 'N', 'F', 'H'],
    ];

    for line in input.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let parsed_line = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");

        let mut items = parsed_line.split_whitespace();
        let count = i32::from_str(items.next().unwrap()).unwrap();
        let from_column = i32::from_str(items.next().unwrap()).unwrap();
        let to_column = i32::from_str(items.next().unwrap()).unwrap();

        let mut number_moved = 0;

        while number_moved < count {
            let val: char = stacks[(from_column - 1) as usize].pop().unwrap();
            stacks[(to_column - 1) as usize].push(val);
            number_moved += 1;
        }
    }

    for stack in stacks {
        println!("{}", stack[stack.len() - 1]);
    }
}

fn part_two() {
    let input = get_data();

    let mut stacks: Vec<Vec<char>> = vec![
        vec!['J', 'H', 'G', 'M', 'Z', 'N', 'T', 'F'],
        vec!['V', 'W', 'J'],
        vec!['G', 'V', 'L', 'J', 'B', 'T', 'H'],
        vec!['B', 'P', 'J', 'N', 'C', 'D', 'V', 'L'],
        vec!['F', 'W', 'S', 'M', 'P', 'R', 'G'],
        vec!['G', 'H', 'C', 'F', 'B', 'N', 'V', 'M'],
        vec!['D', 'H', 'G', 'M', 'R'],
        vec!['H', 'N', 'M', 'V', 'Z', 'D'],
        vec!['G', 'N', 'F', 'H'],
    ];

    for line in input.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let parsed_line = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");

        let mut items = parsed_line.split_whitespace();
        let count = i32::from_str(items.next().unwrap()).unwrap();
        let from_column = i32::from_str(items.next().unwrap()).unwrap();
        let to_column = i32::from_str(items.next().unwrap()).unwrap();

        let mut number_moved = 0;
        let mut tmp_stack = Vec::new();

        while number_moved < count {
            let val: char = stacks[(from_column - 1) as usize].pop().unwrap();
            tmp_stack.push(val);
            number_moved += 1;
        }

        number_moved = 0;

        while number_moved < count {
            let val: char = tmp_stack.pop().unwrap();
            stacks[(to_column - 1) as usize].push(val);
            number_moved += 1;
        }
    }

    for stack in stacks {
        println!("{}", stack[stack.len() - 1]);
    }
}

fn main() {
    // part_one();
    part_two();
}
