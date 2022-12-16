use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::fs;
use std::str::FromStr;

type Point = (i64, i64);

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn parse_line(line: &str) -> (Point, Point) {
    let line = line.strip_prefix("Sensor at x=");

    match line {
        Some(l) => {
            match l
                .split_terminator(": closest beacon is at x=")
                .collect::<Vec<&str>>()[..]
            {
                [sensor_parts, beacon_parts] => (
                    match sensor_parts.split_terminator(", y=").collect::<Vec<&str>>()[..] {
                        [x_str, y_str] => {
                            (i64::from_str(x_str).unwrap(), i64::from_str(y_str).unwrap())
                        }
                        _ => panic!("Unexpected line contents."),
                    },
                    match beacon_parts.split_terminator(", y=").collect::<Vec<&str>>()[..] {
                        [x_str, y_str] => {
                            (i64::from_str(x_str).unwrap(), i64::from_str(y_str).unwrap())
                        }
                        _ => panic!("Unexpected line contents."),
                    },
                ),
                _ => panic!("Unexpected line contents."),
            }
        }
        None => panic!("Unexpected line contents."),
    }
}

fn get_distance_between(start: Point, end: Point) -> u64 {
    ((start.0 - end.0).abs() + (start.1 - end.1).abs())
        .try_into()
        .unwrap()
}

fn is_impossible_beacon_location(
    location: Point,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
    sensors: &Vec<Point>,
    sensor_distances: &HashMap<Point, u64>,
) -> bool {
    let mut point_queue: VecDeque<Point> = VecDeque::new();
    let mut visited_points: Vec<Point> = Vec::new();
    let max_distance = *sensor_distances.values().max().unwrap();

    point_queue.push_back(location);

    loop {
        match point_queue.pop_front() {
            None => {
                return false;
            }
            Some(current_point) => {
                let current_distance = get_distance_between(current_point, location);
                if max_distance < current_distance {
                    continue;
                }

                if sensors.contains(&current_point)
                    && get_distance_between(current_point, location)
                        < *sensor_distances.get(&current_point).unwrap()
                {
                    return true;
                }

                visited_points.push(current_point);

                if current_point.0 > min_x {
                    point_queue.push_back((current_point.0 - 1, current_point.1));
                }

                if current_point.1 > min_y {
                    point_queue.push_back((current_point.0, current_point.1 - 1));
                }

                if current_point.0 < max_x {
                    point_queue.push_back((current_point.0 + 1, current_point.1));
                }

                if current_point.1 < max_y {
                    point_queue.push_back((current_point.0, current_point.1 + 1));
                }
            }
        }
    }
}

fn part_one() {
    let mut sensors: Vec<Point> = Vec::new();
    let mut b: Vec<Point> = Vec::new();
    let mut sensor_beacon_distances: HashMap<Point, u64> = HashMap::new();
    for (sensor, beacon) in get_data().lines().map(|l| parse_line(l)) {
        sensors.push(sensor);

        if !b.contains(&beacon) {
            b.push(beacon);
        }
        sensor_beacon_distances.insert(sensor, get_distance_between(sensor, beacon));
    }
    let beacons: Vec<Point> = b;
    let min_x = sensors
        .iter()
        .map(|s| s.0)
        .chain(beacons.iter().map(|b| b.0))
        .min()
        .unwrap();
    let min_y = sensors
        .iter()
        .map(|s| s.1)
        .chain(beacons.iter().map(|b| b.1))
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.0)
        .chain(beacons.iter().map(|b| b.0))
        .max()
        .unwrap();
    let max_y = sensors
        .iter()
        .map(|s| s.1)
        .chain(beacons.iter().map(|b| b.1))
        .max()
        .unwrap();

    let mut impossible_count = 0;
    let row_index = 2000000;
    // let row_index = 10;
    //
    sensors.sort_by(|a, b| (a.1 - row_index).abs().cmp(&(b.1 - row_index).abs()));

    for x in (min_x - 1000000)..(max_x + 1000000) {
        for sensor in &sensors {
            let distance_to_sensor = get_distance_between((x, row_index), *sensor);
            let sensor_to_beacon_distance = *sensor_beacon_distances.get(&sensor).unwrap();

            if distance_to_sensor <= sensor_to_beacon_distance {
                impossible_count += 1;
                break;
            }
        }
    }

    println!(
        "{} {}",
        impossible_count,
        impossible_count
            - beacons
                .iter()
                .fold(0, |acc, b| if b.1 == row_index { acc + 1 } else { acc })
    );
}

fn part_two() {
    let mut sensors: Vec<Point> = Vec::new();
    let mut b: Vec<Point> = Vec::new();
    let mut sensor_beacon_distances: HashMap<Point, u64> = HashMap::new();
    for (sensor, beacon) in get_data().lines().map(|l| parse_line(l)) {
        sensors.push(sensor);

        if !b.contains(&beacon) {
            b.push(beacon);
        }
        sensor_beacon_distances.insert(sensor, get_distance_between(sensor, beacon));
    }
    let beacons: Vec<Point> = b;
    let min_x = sensors
        .iter()
        .map(|s| s.0)
        .chain(beacons.iter().map(|b| b.0))
        .min()
        .unwrap();
    let min_y = sensors
        .iter()
        .map(|s| s.1)
        .chain(beacons.iter().map(|b| b.1))
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.0)
        .chain(beacons.iter().map(|b| b.0))
        .max()
        .unwrap();
    let max_y = sensors
        .iter()
        .map(|s| s.1)
        .chain(beacons.iter().map(|b| b.1))
        .max()
        .unwrap();

    let mut point_count = 0;
    for sensor in &sensors {
        //&sensors {
        println!("checking sensor {:?}", sensor);
        let sensor_to_beacon_distance: i64 =
            TryInto::<i64>::try_into(*sensor_beacon_distances.get(&sensor).unwrap()).unwrap();

        let x_coords = (0..(sensor_to_beacon_distance + 1))
            .chain((1..(sensor_to_beacon_distance + 2)).rev())
            .chain((-(sensor_to_beacon_distance)..1).rev())
            .chain(-(sensor_to_beacon_distance + 1)..0);

        let y_coords = ((1..(sensor_to_beacon_distance + 2)).rev())
            .chain((-(sensor_to_beacon_distance)..1).rev())
            .chain(-(sensor_to_beacon_distance + 1)..0)
            .chain(0..(sensor_to_beacon_distance + 1));

        let mut is_beacon = true;
        for (x, y) in x_coords.zip(y_coords) {
            let location = (sensor.0 + x, sensor.1 + y);

            if location.0 >= 0 && location.0 <= 4000000 && location.1 >= 0 && location.1 <= 4000000
            {
                for sensor in &sensors {
                    if get_distance_between(location, *sensor)
                        <= *sensor_beacon_distances.get(sensor).unwrap()
                    {
                        is_beacon = false;
                        break;
                    }
                }

                if is_beacon {
                    println!("beacon located at {:?}", location);
                    break;
                }
            }
            is_beacon = true;

            point_count += 1;

            if point_count % 1000000 == 0 {
                println!("checked {} points", point_count);
            }
        }
    }
}

fn main() {
    part_two();
}
