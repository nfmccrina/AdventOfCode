use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Node {
    x: u32,
    y: u32,
    height: u8,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self.height + 97u8))
    }
}

impl Node {
    fn new(x: u32, y: u32, height: u8) -> Self {
        Self { x, y, height }
    }

    fn location(&self) -> (u32, u32) {
        (self.y, self.x)
    }

    fn height(&self) -> u8 {
        self.height
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
    let mut start_node: Option<(u32, u32)> = None;
    let mut end_node: Option<(u32, u32)> = None;
    let nodes: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, ch)| {
                    if *ch == 83u8 {
                        start_node = Some((row.try_into().unwrap(), col.try_into().unwrap()));
                    }
                    if *ch == 69u8 {
                        end_node = Some((row.try_into().unwrap(), col.try_into().unwrap()));
                    }
                    Node::new(
                        col.try_into().unwrap(),
                        row.try_into().unwrap(),
                        if *ch == 83u8 {
                            0u8
                        } else {
                            if *ch == 69u8 {
                                25u8
                            } else {
                                ch - 97u8
                            }
                        },
                    )
                })
                .collect()
        })
        .collect();
    let mut distances: HashMap<(u32, u32), u32> = HashMap::new();
    let mut node_queue: HashSet<(u32, u32)> = HashSet::new();
    let mut prev: HashMap<(u32, u32), Option<(u32, u32)>> = HashMap::new();
    let mut shortest_distance = u32::MAX;

    for row in &nodes {
        for col in row {
            if col.location() == start_node.unwrap() {
                distances.insert(col.location(), 0);
            } else {
                distances.insert(col.location(), u32::MAX);
            }

            prev.insert(col.location(), None);
            node_queue.insert(col.location());
        }
    }

    loop {
        if node_queue.len() == 0 {
            break;
        }

        let mut closest_node: Option<(u32, u32)> = None;
        let mut closest_node_distance: u32 = u32::MAX;

        for key in &node_queue {
            if distances[key] <= closest_node_distance {
                closest_node = Some(*key);
                closest_node_distance = distances[key];
            }
        }

        if closest_node.unwrap() == end_node.unwrap() {
            shortest_distance = distances[&closest_node.unwrap()];
            break;
        }

        node_queue.remove(&closest_node.unwrap());

        let mut neighbors: Vec<(u32, u32)> = Vec::new();
        let (y, x) = closest_node.unwrap();

        //println!("{}, {}", y, x);
        //println!("{:?}", node_queue);
        // println!("{:?}", distances);

        if y > 0 {
            if node_queue.contains(&(y - 1, x))
                && (nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height() + 1)
                    >= nodes[usize::try_from(y - 1).unwrap()][usize::try_from(x).unwrap()].height()
            {
                neighbors.push((y - 1, x));
            }
        }
        if y < u32::try_from(nodes.len()).unwrap() - 1 {
            if node_queue.contains(&(y + 1, x))
                && (nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height() + 1)
                    >= nodes[usize::try_from(y + 1).unwrap()][usize::try_from(x).unwrap()].height()
            {
                neighbors.push((y + 1, x));
            }
        }

        if x > 0 {
            if node_queue.contains(&(y, x - 1))
                && (nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height() + 1)
                    >= nodes[usize::try_from(y).unwrap()][usize::try_from(x - 1).unwrap()].height()
            {
                neighbors.push((y, x - 1));
            }
        }
        if x < u32::try_from(nodes[0].len()).unwrap() - 1 {
            if node_queue.contains(&(y, x + 1))
                && (nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height() + 1)
                    >= nodes[usize::try_from(y).unwrap()][usize::try_from(x + 1).unwrap()].height()
            {
                neighbors.push((y, x + 1));
            }
        }

        //println!("{:?}", neighbors);
        for n in neighbors {
            let alt = distances.get(&(y, x)).unwrap() + 1;

            if alt < *(distances.get(&n).unwrap()) {
                distances.insert(n, alt);
                prev.insert(n, Some((y, x)));
            }
        }
    }

    println!("shortest: {}", shortest_distance);
}

fn part_two() {
    let input = get_data();
    let mut start_node: Option<(u32, u32)> = None;
    let nodes: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, ch)| {
                    if *ch == 69u8 {
                        start_node = Some((row.try_into().unwrap(), col.try_into().unwrap()));
                    }
                    Node::new(
                        col.try_into().unwrap(),
                        row.try_into().unwrap(),
                        if *ch == 83u8 {
                            0u8
                        } else {
                            if *ch == 69u8 {
                                25u8
                            } else {
                                ch - 97u8
                            }
                        },
                    )
                })
                .collect()
        })
        .collect();
    let mut distances: HashMap<(u32, u32), u32> = HashMap::new();
    let mut node_queue: HashSet<(u32, u32)> = HashSet::new();
    let mut prev: HashMap<(u32, u32), Option<(u32, u32)>> = HashMap::new();
    let mut shortest_distance = u32::MAX;

    for row in &nodes {
        for col in row {
            if col.location() == start_node.unwrap() {
                distances.insert(col.location(), 0);
            } else {
                distances.insert(col.location(), u32::MAX);
            }

            prev.insert(col.location(), None);
            node_queue.insert(col.location());
        }
    }

    loop {
        if node_queue.len() == 0 {
            break;
        }

        let mut closest_node: Option<(u32, u32)> = None;
        let mut closest_node_distance: u32 = u32::MAX;

        for key in &node_queue {
            if distances[key] <= closest_node_distance {
                closest_node = Some(*key);
                closest_node_distance = distances[key];
            }
        }

        let (y, x) = closest_node.unwrap();
        if nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height() == 0 {
            shortest_distance = distances[&closest_node.unwrap()];
            break;
        }

        node_queue.remove(&closest_node.unwrap());

        let mut neighbors: Vec<(u32, u32)> = Vec::new();

        //println!("{}, {}", y, x);
        //println!("{:?}", node_queue);
        // println!("{:?}", distances);

        if y > 0 {
            if node_queue.contains(&(y - 1, x))
                && nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height()
                    <= (nodes[usize::try_from(y - 1).unwrap()][usize::try_from(x).unwrap()]
                        .height()
                        + 1)
            {
                neighbors.push((y - 1, x));
            }
        }
        if y < u32::try_from(nodes.len()).unwrap() - 1 {
            if node_queue.contains(&(y + 1, x))
                && nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height()
                    <= (nodes[usize::try_from(y + 1).unwrap()][usize::try_from(x).unwrap()]
                        .height()
                        + 1)
            {
                neighbors.push((y + 1, x));
            }
        }

        if x > 0 {
            if node_queue.contains(&(y, x - 1))
                && nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height()
                    <= (nodes[usize::try_from(y).unwrap()][usize::try_from(x - 1).unwrap()]
                        .height()
                        + 1)
            {
                neighbors.push((y, x - 1));
            }
        }
        if x < u32::try_from(nodes[0].len()).unwrap() - 1 {
            if node_queue.contains(&(y, x + 1))
                && nodes[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].height()
                    <= (nodes[usize::try_from(y).unwrap()][usize::try_from(x + 1).unwrap()]
                        .height()
                        + 1)
            {
                neighbors.push((y, x + 1));
            }
        }

        //println!("{:?}", neighbors);
        for n in neighbors {
            let alt = distances.get(&(y, x)).unwrap() + 1;

            if alt < *(distances.get(&n).unwrap()) {
                distances.insert(n, alt);
                prev.insert(n, Some((y, x)));
            }
        }
    }

    println!("shortest: {}", shortest_distance);
}

fn main() {
    // part_two();
    part_one();
}
