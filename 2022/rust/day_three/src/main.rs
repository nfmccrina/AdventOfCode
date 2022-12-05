use std::fs;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn get_compartments(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn part_one() {
    let input = get_data();
    let mut priorities = Vec::new();

    for line in input.lines() {
        let mut priority = 0;
        let compartments = get_compartments(line);

        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".char_indices() {
            if compartments.0.contains(ch.1) && compartments.1.contains(ch.1) {
                priority = ch.0 as i32 + 1;
                break;
            }
        }

        priorities.push(priority);
    }

    println!("Sum of priorities is {}", priorities.iter().sum::<i32>());
}

fn get_groups(input: &str) -> Vec<(&str, &str, &str)> {
    let mut groups = Vec::new();
    let mut current_group: (&str, &str, &str) = ("", "", "");
    let mut current_index = 0;

    for line in input.lines() {
        if current_index % 3 == 0 {
            if current_index != 0 {
                groups.push(current_group);
            }

            current_group.0 = line;
        }

        if current_index % 3 == 1 {
            current_group.1 = line;
        }

        if current_index % 3 == 2 {
            current_group.2 = line;
        }

        current_index = current_index + 1;
    }

    groups.push(current_group);

    groups
}

fn part_two() {
    let input = get_data();
    let groups = get_groups(input.as_str());
    let mut priorities: Vec<i32> = Vec::new();

    for group in groups {
        let mut priority = 0;
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".char_indices() {
            if group.0.contains(ch.1) && group.1.contains(ch.1) && group.2.contains(ch.1) {
                priority = ch.0 as i32 + 1;
                break;
            }
        }
        priorities.push(priority);
    }

    println!("Sum of priorities is {}", priorities.iter().sum::<i32>());
}

fn main() {
    // part_one();
    part_two();
}
