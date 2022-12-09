use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Left { distance: u32 },
    Right { distance: u32 },
    Up { distance: u32 },
    Down { distance: u32 },
}

impl Move {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let distance = match u32::from_str(parts[1]) {
            Ok(u) => u,
            Err(_) => panic!("Unable to parse {} as u32.", parts[1]),
        };
        match parts[0] {
            "L" => Self::Left { distance },
            "R" => Self::Right { distance },
            "U" => Self::Up { distance },
            "D" => Self::Down { distance },
            _ => panic!("Unable to parse {} as a Direction.", parts[0]),
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

fn part_one() {
    let input = get_data();
    let moves = input.lines().map(|line| Move::from(line));
    let mut current_head: (i32, i32) = (0, 0);
    let mut current_tail: (i32, i32) = (0, 0);
    let mut tail_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for m in moves {
        match m {
            Move::Left { distance } => {
                current_head = (current_head.0 - (distance as i32), current_head.1);

                for i in ((current_head.0 + 1)..current_tail.0).rev() {
                    current_tail = (i, current_tail.1 + (current_head.1 - current_tail.1));
                    tail_positions.push(current_tail);
                }
            }
            Move::Right { distance } => {
                current_head = (current_head.0 + (distance as i32), current_head.1);

                for i in (current_tail.0 + 1)..current_head.0 {
                    current_tail = (i, current_tail.1 + (current_head.1 - current_tail.1));
                    tail_positions.push(current_tail);
                }
            }
            Move::Up { distance } => {
                current_head = (current_head.0, current_head.1 + (distance as i32));

                for i in (current_tail.1 + 1)..current_head.1 {
                    current_tail = (current_tail.0 + (current_head.0 - current_tail.0), i);
                    tail_positions.push(current_tail);
                }
            }
            Move::Down { distance } => {
                current_head = (current_head.0, current_head.1 - (distance as i32));

                for i in ((current_head.1 + 1)..current_tail.1).rev() {
                    current_tail = (current_tail.0 + (current_head.0 - current_tail.0), i);
                    tail_positions.push(current_tail);
                }
            }
        }
    }

    tail_positions.sort();
    tail_positions.dedup();
    println!("number of visited positions: {}", tail_positions.len());
}

fn move_knot(start_position: (i32, i32), head_position: (i32, i32)) -> (i32, i32) {
    let new_position: (i32, i32);
    if (head_position.0 - start_position.0).abs() <= 1
        && (head_position.1 - start_position.1).abs() <= 1
    {
        new_position = start_position;
    } else if head_position.0 == start_position.0 && (head_position.1 - start_position.1).abs() > 1
    {
        if head_position.1 > start_position.1 {
            new_position = (start_position.0, start_position.1 + 1);
        } else {
            new_position = (start_position.0, start_position.1 - 1);
        }
    } else if head_position.1 == start_position.1 && (head_position.0 - start_position.0).abs() > 1
    {
        if head_position.0 > start_position.0 {
            new_position = (start_position.0 + 1, start_position.1);
        } else {
            new_position = (start_position.0 - 1, start_position.1);
        }
    } else if head_position.0 < start_position.0 && head_position.1 < start_position.1 {
        new_position = (start_position.0 - 1, start_position.1 - 1);
    } else if head_position.0 < start_position.0 && head_position.1 > start_position.1 {
        new_position = (start_position.0 - 1, start_position.1 + 1);
    } else if head_position.0 > start_position.0 && head_position.1 < start_position.1 {
        new_position = (start_position.0 + 1, start_position.1 - 1);
    } else if head_position.0 > start_position.0 && head_position.1 > start_position.1 {
        new_position = (start_position.0 + 1, start_position.1 + 1);
    } else {
        panic!(
            "unexpected combination of positions: start -> ({}, {}), head -> ({}, {})",
            start_position.0, start_position.1, head_position.0, head_position.1
        );
    }

    new_position
}

fn part_two() {
    let input = get_data();
    let moves = input.lines().map(|line| Move::from(line));
    let mut knot_positions: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited_tail_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for m in moves {
        match m {
            Move::Left { distance } => {
                for _ in 0..distance {
                    knot_positions[0] = (knot_positions[0].0 - 1, knot_positions[0].1);

                    for i in 1..knot_positions.len() {
                        knot_positions[i] = move_knot(knot_positions[i], knot_positions[i - 1]);
                    }

                    visited_tail_positions.push(knot_positions[knot_positions.len() - 1]);
                }
            }
            Move::Right { distance } => {
                for _ in 0..distance {
                    knot_positions[0] = (knot_positions[0].0 + 1, knot_positions[0].1);

                    for i in 1..knot_positions.len() {
                        knot_positions[i] = move_knot(knot_positions[i], knot_positions[i - 1]);
                    }

                    visited_tail_positions.push(knot_positions[knot_positions.len() - 1]);
                }
            }
            Move::Up { distance } => {
                for _ in 0..distance {
                    knot_positions[0] = (knot_positions[0].0, knot_positions[0].1 + 1);

                    for i in 1..knot_positions.len() {
                        knot_positions[i] = move_knot(knot_positions[i], knot_positions[i - 1]);
                    }

                    visited_tail_positions.push(knot_positions[knot_positions.len() - 1]);
                }
            }
            Move::Down { distance } => {
                for _ in 0..distance {
                    knot_positions[0] = (knot_positions[0].0, knot_positions[0].1 - 1);

                    for i in 1..knot_positions.len() {
                        knot_positions[i] = move_knot(knot_positions[i], knot_positions[i - 1]);
                    }

                    visited_tail_positions.push(knot_positions[knot_positions.len() - 1]);
                }
            }
        }
    }

    visited_tail_positions.sort();
    visited_tail_positions.dedup();
    println!(
        "number of visited positions: {}",
        visited_tail_positions.len()
    );
}

fn main() {
    part_two();
}
