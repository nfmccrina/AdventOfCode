use std::fs;
use std::str::FromStr;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let input = get_data();

    let mut elf_calorie_list = Vec::new();
    let mut current_elf_total: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            elf_calorie_list.push(current_elf_total);
            current_elf_total = 0;
            continue;
        }

        current_elf_total = current_elf_total + i32::from_str(line).unwrap();
    }

    let mut highest_calorie_total = 0;
    let mut highest_calorie_index = 0;

    for (index, calorie_total) in elf_calorie_list.iter().enumerate() {
        if *calorie_total > highest_calorie_total {
            highest_calorie_total = *calorie_total;
            highest_calorie_index = index;
        }
    }

    println!(
        "The highest calorie total is {}, at index {}",
        highest_calorie_total, highest_calorie_index
    );
}

fn part_two() {
    let input = get_data();

    let mut elf_calorie_list = Vec::new();
    let mut current_elf_total: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            elf_calorie_list.push(current_elf_total);
            current_elf_total = 0;
            continue;
        }

        current_elf_total = current_elf_total + i32::from_str(line).unwrap();
    }

    let mut highest_calorie_totals = vec![0, 0, 0];

    for calorie_total in elf_calorie_list {
        let mut index = 0;
        while index < 3 {
            if calorie_total > highest_calorie_totals[index] {
                highest_calorie_totals.insert(index, calorie_total);
                highest_calorie_totals.pop();
                break;
            }

            index = index + 1;
        }
    }

    println!(
        "The sum of the highest calorie totals is {}",
        highest_calorie_totals.iter().sum::<i32>()
    );
}

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}
