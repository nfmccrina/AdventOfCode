use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

#[derive(Clone)]
enum Object {
    None,
    Wall,
    Sand,
    SandSource,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Point, ParseIntError> {
        let (x_str, y_str) = s.split_once(',').unwrap_or(("", ""));
        Ok(Point::new(i32::from_str(x_str)?, i32::from_str(y_str)?))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split_terminator(" -> ")
        .map(|pair| Point::from_str(pair).unwrap())
        .collect()
}

fn get_intervening_points(first: &Point, second: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if first.x() == second.x() {
        // moving in y dimension
        let range: std::ops::Range<i32>;
        if first.y() < second.y() {
            range = first.y()..(second.y() + 1);
        } else {
            range = second.y()..(first.y() + 1);
        }

        for i in range {
            points.push(Point::new(first.x(), i));
        }
    } else {
        // moving in x dimension
        let range: std::ops::Range<i32>;
        if first.x() < second.x() {
            range = first.x()..(second.x() + 1);
        } else {
            range = second.x()..(first.x() + 1);
        }

        for i in range {
            points.push(Point::new(i, first.y()));
        }
    }

    points
}

fn print_space(space: &Vec<Vec<Object>>, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    let mut s = String::new();

    println!(
        "{:>8}{:width$}  {}",
        min_x,
        " ",
        max_x,
        width = (max_x - min_x) + 1
    );
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            s.push(match space[y][x] {
                Object::None => '.',
                Object::Wall => '#',
                Object::Sand => 'o',
                Object::SandSource => '+',
            });
        }

        println!("{:>8} {} {}", y, s, y);
        s.clear();
    }
    println!(
        "{:>8}{:width$}  {}",
        min_x,
        " ",
        max_x,
        width = (max_x - min_x) + 1
    );
}

fn get_max_x(points: &Vec<Vec<Point>>) -> i32 {
    points.iter().fold(i32::MIN, |col_acc, col| {
        col.iter().fold(col_acc, |point_acc, point| {
            if point.x() > point_acc {
                point.x()
            } else {
                point_acc
            }
        })
    })
}

fn get_max_y(points: &Vec<Vec<Point>>) -> i32 {
    points.iter().fold(i32::MIN, |col_acc, col| {
        col.iter().fold(col_acc, |point_acc, point| {
            if point.y() > point_acc {
                point.y()
            } else {
                point_acc
            }
        })
    })
}

fn initialize_space(max_x: i32, max_y: i32, walls: &Vec<Vec<Point>>) -> Vec<Vec<Object>> {
    let mut space = vec![vec![Object::None; max_x.try_into().unwrap()]; max_y.try_into().unwrap()];

    for wall in walls {
        for (start, end) in wall.iter().zip(wall.iter().skip(1)) {
            for point in get_intervening_points(start, end) {
                space[TryInto::<usize>::try_into(point.y()).unwrap()]
                    [TryInto::<usize>::try_into(point.x()).unwrap()] = Object::Wall;
            }
        }
    }

    space
}

fn part_one() {
    let walls: Vec<Vec<Point>> = get_data().lines().map(|line| parse_line(line)).collect();

    let max_x = get_max_x(&walls);
    let max_y = get_max_y(&walls);

    let mut space = initialize_space(max_x + 1, max_y + 1, &walls);

    space[0][500] = Object::SandSource;

    let mut sand_count = 0;
    let mut current_sand_position = Point::new(500, 0);
    let mut at_rest = false;
    'sand: loop {
        sand_count += 1;
        'sand_lifecycle: loop {
            if at_rest {
                at_rest = false;
                current_sand_position = Point::new(500, 0);
                break;
            }

            if current_sand_position.y() >= max_y {
                println!(
                    "Sand has gone into the void. {} units of sand at rest.",
                    sand_count - 1
                );
                break 'sand;
            }

            for x in [
                current_sand_position.x(),
                current_sand_position.x() - 1,
                current_sand_position.x() + 1,
            ] {
                match space[TryInto::<usize>::try_into(current_sand_position.y() + 1).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()]
                {
                    Object::None => {
                        if current_sand_position.x() != 500 || current_sand_position.y() != 0 {
                            space[TryInto::<usize>::try_into(current_sand_position.y()).unwrap()]
                                [TryInto::<usize>::try_into(current_sand_position.x()).unwrap()] =
                                Object::None;
                        }
                        current_sand_position = Point::new(x, current_sand_position.y() + 1);
                        space[TryInto::<usize>::try_into(current_sand_position.y()).unwrap()]
                            [TryInto::<usize>::try_into(current_sand_position.x()).unwrap()] =
                            Object::Sand;
                        continue 'sand_lifecycle;
                    }
                    _ => {
                        continue;
                    }
                }
            }

            at_rest = true;
        }
    }
}

fn part_two() {
    let walls: Vec<Vec<Point>> = get_data().lines().map(|line| parse_line(line)).collect();

    let max_x = get_max_x(&walls);
    let max_y = get_max_y(&walls) + 2;

    let mut space = initialize_space(
        if (max_x + 1) > (501 + max_y) {
            max_x + 1
        } else {
            501 + max_y
        },
        max_y + 1,
        &walls,
    );

    for i in 0..space[0].len() {
        space[TryInto::<usize>::try_into(max_y).unwrap()][i] = Object::Wall;
    }

    space[0][500] = Object::SandSource;

    let mut sand_count = 0;
    let mut current_sand_position = Point::new(500, 0);
    let mut at_rest = false;
    'sand: loop {
        sand_count += 1;
        'sand_lifecycle: loop {
            if at_rest {
                if current_sand_position.x() == 500 && current_sand_position.y() == 0 {
                    println!("Sand is blocked. Units of sand at rest: {}", sand_count);
                    break 'sand;
                }
                at_rest = false;
                current_sand_position = Point::new(500, 0);
                break;
            }

            if current_sand_position.y() >= max_y {
                println!(
                    "Sand has gone into the void. {} units of sand at rest.",
                    sand_count - 1
                );
                panic!("Sand shouldn't go into the void in part two.");
            }

            for x in [
                current_sand_position.x(),
                current_sand_position.x() - 1,
                current_sand_position.x() + 1,
            ] {
                match space[TryInto::<usize>::try_into(current_sand_position.y() + 1).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()]
                {
                    Object::None => {
                        if current_sand_position.x() != 500 || current_sand_position.y() != 0 {
                            space[TryInto::<usize>::try_into(current_sand_position.y()).unwrap()]
                                [TryInto::<usize>::try_into(current_sand_position.x()).unwrap()] =
                                Object::None;
                        }
                        current_sand_position = Point::new(x, current_sand_position.y() + 1);
                        space[TryInto::<usize>::try_into(current_sand_position.y()).unwrap()]
                            [TryInto::<usize>::try_into(current_sand_position.x()).unwrap()] =
                            Object::Sand;
                        continue 'sand_lifecycle;
                    }
                    _ => {
                        continue;
                    }
                }
            }

            at_rest = true;
        }
    }
}

fn main() {
    part_two();
}
