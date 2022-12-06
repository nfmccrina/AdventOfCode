use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn is_match_part_two(current_group: &HashMap<char, u32>) -> bool {
    for (_, value) in current_group {
        if *value > 1 {
            return true;
        }
    }

    false
}

fn is_match(current_group: u32) -> bool {
    let bytes: (u8, u8, u8, u8) = (
        ((current_group & 0xFF000000) >> 24) as u8,
        ((current_group & 0x00FF0000) >> 16) as u8,
        ((current_group & 0x0000FF00) >> 8) as u8,
        (current_group & 0x000000FF) as u8,
    );

    bytes.0 == bytes.1
        || bytes.0 == bytes.2
        || bytes.0 == bytes.3
        || bytes.1 == bytes.0
        || bytes.1 == bytes.2
        || bytes.1 == bytes.3
        || bytes.2 == bytes.0
        || bytes.2 == bytes.1
        || bytes.2 == bytes.3
        || bytes.3 == bytes.0
        || bytes.3 == bytes.1
        || bytes.3 == bytes.2
}

fn part_one() {
    let input = get_data();
    let input_bytes = input.into_bytes();
    let mut current_group: u32 = 0;
    let mut index = 0;
    let mut next_byte: u8 = input_bytes[index];

    while index < 4 {
        current_group <<= 8;
        current_group |= next_byte as u32;
        index += 1;
        next_byte = input_bytes[index];
    }

    while is_match(current_group) {
        current_group <<= 8;
        current_group |= next_byte as u32;
        index += 1;
        next_byte = input_bytes[index];
    }

    println!("Marker appears after position {}", index);
}

fn part_two() {
    let input = get_data();
    let mut current_group_hash: HashMap<char, u32> = HashMap::new();
    let mut current_group: VecDeque<char> = VecDeque::new();

    for (index, ch) in input.char_indices() {
        current_group.push_back(ch);
        let new_hash_entry = current_group_hash.entry(ch).or_insert(0);
        *new_hash_entry += 1;

        if index > 13 {
            let discarded_char = current_group.pop_front().unwrap();
            current_group_hash
                .entry(discarded_char)
                .and_modify(|item| *item -= 1)
                .or_insert(0);

            if !is_match_part_two(&current_group_hash) {
                println!("Marker appears after position {}", index + 1);
                break;
            }
        }
    }
}

fn main() {
    // part_one();
    part_two();
}
