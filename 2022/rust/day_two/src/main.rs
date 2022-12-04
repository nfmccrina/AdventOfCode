use std::fs;

//enum GameResult {
//    PlayerWin,
//    ElfWin,
//    Draw,
//}
//
//enum PlayerSelection {
//    X,
//    Y,
//    Z,
//}
//
//enum ElfSelection {
//    A,
//    B,
//    C,
//}
//
//fn parse_input_line(line: &str) -> (PlayerSelection, ElfSelection) {
//    let mut chars = line.chars();
//
//    let elf_selection = parse_elf_selection(chars.next());
//    chars.next();
//    let player_selection = parse_player_selection(chars.next());
//
//    return (player_selection, elf_selection);
//}
//
//fn parse_player_selection(c: Option<char>) -> PlayerSelection {
//    return match c {
//        Some('X') => PlayerSelection::X,
//        Some('Y') => PlayerSelection::Y,
//        Some('Z') => PlayerSelection::Z,
//        _ => panic!("{:?} does not match a valid player selection.", c),
//    };
//}
//
//fn parse_elf_selection(c: Option<char>) -> ElfSelection {
//    return match c {
//        Some('A') => ElfSelection::A,
//        Some('B') => ElfSelection::B,
//        Some('C') => ElfSelection::C,
//        _ => panic!("{:?} does not match a valid elf selection.", c),
//    };
//}
//
//fn get_result(selections: &(PlayerSelection, ElfSelection)) -> GameResult {
//    return match selections {
//        (PlayerSelection::X, ElfSelection::A) => GameResult::Draw,
//        (PlayerSelection::X, ElfSelection::B) => GameResult::ElfWin,
//        (PlayerSelection::X, ElfSelection::C) => GameResult::PlayerWin,
//        (PlayerSelection::Y, ElfSelection::A) => GameResult::PlayerWin,
//        (PlayerSelection::Y, ElfSelection::B) => GameResult::Draw,
//        (PlayerSelection::Y, ElfSelection::C) => GameResult::ElfWin,
//        (PlayerSelection::Z, ElfSelection::A) => GameResult::ElfWin,
//        (PlayerSelection::Z, ElfSelection::B) => GameResult::PlayerWin,
//        (PlayerSelection::Z, ElfSelection::C) => GameResult::Draw,
//    };
//}
//
//fn get_result_score(result: GameResult) -> i32 {
//    return match result {
//        GameResult::PlayerWin => 6,
//        GameResult::ElfWin => 0,
//        GameResult::Draw => 3,
//    };
//}
//
//fn get_selection_score(selection: PlayerSelection) -> i32 {
//    return match selection {
//        PlayerSelection::X => 1,
//        PlayerSelection::Y => 2,
//        PlayerSelection::Z => 3,
//    };
//}
//
//fn part_one() {
//    let input = get_data();
//    let mut scores: Vec<i32> = Vec::new();
//
//    for line in input.lines() {
//        let selections: (PlayerSelection, ElfSelection) = parse_input_line(line);
//
//        let final_score =
//            get_result_score(get_result(&selections)) + get_selection_score(selections.0);
//
//        scores.push(final_score);
//    }
//
//    println!("Total score is {}", scores.iter().sum::<i32>());
//}

enum GameResult {
    PlayerWin,
    ElfWin,
    Draw,
}

enum Selection {
    Rock,
    Paper,
    Scissors,
}

fn parse_input_line(line: &str) -> (Selection, Selection) {
    let mut chars = line.chars();

    let elf_selection = parse_elf_selection(chars.next());
    chars.next();
    let player_selection = parse_player_selection(chars.next(), &elf_selection);

    return (player_selection, elf_selection);
}

fn parse_player_selection(c: Option<char>, elf_selection: &Selection) -> Selection {
    return match c {
        Some('X') => match elf_selection {
            Selection::Rock => Selection::Scissors,
            Selection::Paper => Selection::Rock,
            Selection::Scissors => Selection::Paper,
        },
        Some('Y') => match elf_selection {
            Selection::Rock => Selection::Rock,
            Selection::Paper => Selection::Paper,
            Selection::Scissors => Selection::Scissors,
        },
        Some('Z') => match elf_selection {
            Selection::Rock => Selection::Paper,
            Selection::Paper => Selection::Scissors,
            Selection::Scissors => Selection::Rock,
        },
        _ => panic!("{:?} does not match a valid player selection.", c),
    };
}

fn parse_elf_selection(c: Option<char>) -> Selection {
    return match c {
        Some('A') => Selection::Rock,
        Some('B') => Selection::Paper,
        Some('C') => Selection::Scissors,
        _ => panic!("{:?} does not match a valid elf selection.", c),
    };
}

fn get_result(selections: &(Selection, Selection)) -> GameResult {
    return match selections {
        (Selection::Rock, Selection::Rock) => GameResult::Draw,
        (Selection::Rock, Selection::Paper) => GameResult::ElfWin,
        (Selection::Rock, Selection::Scissors) => GameResult::PlayerWin,
        (Selection::Paper, Selection::Rock) => GameResult::PlayerWin,
        (Selection::Paper, Selection::Paper) => GameResult::Draw,
        (Selection::Paper, Selection::Scissors) => GameResult::ElfWin,
        (Selection::Scissors, Selection::Rock) => GameResult::ElfWin,
        (Selection::Scissors, Selection::Paper) => GameResult::PlayerWin,
        (Selection::Scissors, Selection::Scissors) => GameResult::Draw,
    };
}

fn get_result_score(result: GameResult) -> i32 {
    return match result {
        GameResult::PlayerWin => 6,
        GameResult::ElfWin => 0,
        GameResult::Draw => 3,
    };
}

fn get_selection_score(selection: Selection) -> i32 {
    return match selection {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    };
}

fn part_two() {
    let input = get_data();
    let mut scores: Vec<i32> = Vec::new();

    for line in input.lines() {
        let selections: (Selection, Selection) = parse_input_line(line);

        let final_score =
            get_result_score(get_result(&selections)) + get_selection_score(selections.0);

        scores.push(final_score);
    }

    println!("Total score is {}", scores.iter().sum::<i32>());
}

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn main() {
    part_two();
}
