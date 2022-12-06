use std::fs;
use std::str::FromStr;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn get_assignments(line: &str) -> (&str, &str) {
    let split_result = line.split_once(',');

    match split_result {
        Some(_) => split_result.unwrap(),
        None => panic!("unable to split {:?}", line),
    }
}

fn get_sections(assignment: &str) -> (i32, i32) {
    let split_result = assignment.split_once('-').unwrap_or_else(|| ("0", "0"));

    (
        i32::from_str(split_result.0).unwrap(),
        i32::from_str(split_result.1).unwrap(),
    )
}

fn part_one() {
    let input = get_data();
    let mut contained_assignments = 0;

    for line in input.lines() {
        let assignments: (&str, &str) = get_assignments(line);
        let elf_one_sections = get_sections(assignments.0);
        let elf_two_sections = get_sections(assignments.1);

        if (elf_one_sections.0 >= elf_two_sections.0 && elf_one_sections.1 <= elf_two_sections.1)
            || (elf_two_sections.0 >= elf_one_sections.0
                && elf_two_sections.1 <= elf_one_sections.1)
        {
            contained_assignments += 1;
        }
    }

    println!(
        "There are {} overlapping assignment pairs",
        contained_assignments
    );
}

fn part_two() {
    let input = get_data();
    let mut contained_assignments = 0;

    for line in input.lines() {
        let assignments: (&str, &str) = get_assignments(line);
        let elf_one_sections = get_sections(assignments.0);
        let elf_two_sections = get_sections(assignments.1);

        if (elf_one_sections.0 >= elf_two_sections.0 && elf_one_sections.0 <= elf_two_sections.1)
            || (elf_one_sections.1 >= elf_two_sections.0
                && elf_one_sections.1 <= elf_two_sections.1)
            || (elf_two_sections.0 >= elf_one_sections.0
                && elf_two_sections.0 <= elf_one_sections.1)
            || (elf_two_sections.1 >= elf_one_sections.0
                && elf_two_sections.1 <= elf_one_sections.1)
        {
            contained_assignments += 1;
        }
    }

    println!(
        "There are {} overlapping assignment pairs",
        contained_assignments
    );
}

fn main() {
    // part_one();
    part_two();
}
