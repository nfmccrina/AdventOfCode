use std::convert::TryInto;
use std::fs;
use std::str::Lines;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn build_grid(lines: Lines) -> Vec<Vec<u32>> {
    lines
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

fn check_tree(
    grid: &Vec<Vec<u32>>,
    statuses: &mut Vec<Vec<bool>>,
    current_row: usize,
    current_col: usize,
    highest_tree: i32,
) -> i32 {
    let current_tree = grid[current_row][current_col];

    if highest_tree < 0 || current_tree > highest_tree.try_into().unwrap() {
        statuses[current_row][current_col] = true;
        return current_tree.try_into().unwrap();
    }

    highest_tree
}

fn check_grid<F, G>(
    grid: &Vec<Vec<u32>>,
    statuses: &mut Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    row_step: F,
    col_step: G,
    by_row: bool,
) where
    F: Fn(usize) -> usize,
    G: Fn(usize) -> usize,
{
    let mut highest_tree = -1;
    let mut current_row = start_row;
    let mut current_col = start_col;
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    loop {
        loop {
            highest_tree = check_tree(grid, statuses, current_row, current_col, highest_tree);

            if (by_row
                && (current_col <= 0 || current_col >= grid_width - 1)
                && current_col != start_col)
                || (!by_row
                    && (current_row <= 0 || current_row >= grid_height - 1)
                    && current_row != start_row)
            {
                break;
            }

            if by_row {
                current_col = col_step(current_col);
            } else {
                current_row = row_step(current_row);
            }
        }

        if (!by_row
            && (current_col <= 0 || current_col >= grid_width - 1)
            && current_col != start_col)
            || (by_row
                && (current_row <= 0 || current_row >= grid_height - 1)
                && current_row != start_row)
        {
            break;
        }

        if by_row {
            current_row = row_step(current_row);
            current_col = start_col;
        } else {
            current_col = col_step(current_col);
            current_row = start_row;
        }

        highest_tree = -1;
    }
}

fn part_one() {
    let input = get_data();
    let grid = build_grid(input.lines());
    let mut tree_statuses = vec![vec![false; grid[0].len()]; input.lines().count()];

    // left-to-right
    check_grid(&grid, &mut tree_statuses, 0, 0, |r| r + 1, |c| c + 1, true);

    // right-to-left
    check_grid(
        &grid,
        &mut tree_statuses,
        0,
        grid[0].len() - 1,
        |r| r + 1,
        |c| c - 1,
        true,
    );

    // top-to-bottom
    check_grid(&grid, &mut tree_statuses, 0, 0, |r| r + 1, |c| c + 1, false);

    // bottom-to-top
    check_grid(
        &grid,
        &mut tree_statuses,
        grid.len() - 1,
        0,
        |r| r - 1,
        |c| c + 1,
        false,
    );

    let total = tree_statuses.iter().fold(0, |row_accumulator, row| {
        row_accumulator
            + row.iter().fold(0, |col_accumulator, col| match col {
                true => col_accumulator + 1,
                false => col_accumulator,
            })
    });

    println!("total visible = {}", total);
}

fn get_score<F, G>(row: usize, col: usize, grid: &Vec<Vec<u32>>, row_step: F, col_step: G) -> u32
where
    F: Fn(usize) -> usize,
    G: Fn(usize) -> usize,
{
    let mut trees: Vec<u32> = Vec::new();
    let starting_tree = grid[row][col];
    let mut current_tree: u32;
    let mut row_index = row;
    let mut col_index = col;

    loop {
        if (col_step(1) == 1 && (row_index == 0 || row_index >= grid.len() - 1))
            || (row_step(1) == 1 && (col_index == 0 || col_index >= grid[0].len() - 1))
        {
            break;
        }

        row_index = row_step(row_index);
        col_index = col_step(col_index);
        current_tree = grid[row_index][col_index];
        trees.push(current_tree);

        if current_tree >= starting_tree {
            break;
        }
    }

    trees.len().try_into().unwrap()
}

fn part_two() {
    let input = get_data();
    let grid = build_grid(input.lines());

    let highest_score = grid
        .iter()
        .enumerate()
        .fold(0, |row_accumulator, (row, row_list)| {
            let row_score = row_list
                .iter()
                .enumerate()
                .fold(0, |col_accumulator, (col, _)| {
                    let col_score = get_score(row, col, &grid, |r| r, |c| c - 1)
                        * get_score(row, col, &grid, |r| r, |c| c + 1)
                        * get_score(row, col, &grid, |r| r - 1, |c| c)
                        * get_score(row, col, &grid, |r| r + 1, |c| c);

                    if col_score > col_accumulator {
                        return col_score;
                    } else {
                        return col_accumulator;
                    }
                });

            if row_score > row_accumulator {
                return row_score;
            } else {
                return row_accumulator;
            }
        });

    println!("highest score is {}", highest_score);
}

fn main() {
    // part_one();
    part_two();
}
