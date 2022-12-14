use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[derive(Clone)]
enum PacketItem {
    Integer(u32),
    List(Vec<PacketItem>),
}

#[derive(Clone)]
enum Packet {
    Empty,
    Data(Vec<PacketItem>),
}

impl PartialEq for PacketItem {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => left == right,
                _ => false,
            },
            Self::List(left) => match other {
                Self::List(right) => {
                    let zip_result = left
                        .iter()
                        .zip(right.iter())
                        .fold(true, |acc, (lval, rval)| acc && lval.eq(rval));

                    if zip_result {
                        left.len().eq(&right.len())
                    } else {
                        zip_result
                    }
                }
                _ => false,
            },
        }
    }
}

impl Eq for PacketItem {}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Integer(left) => match other {
                Self::Integer(right) => left.cmp(right),
                _ => PacketItem::List(vec![PacketItem::Integer(*left)]).cmp(other),
            },
            Self::List(left) => match other {
                Self::Integer(right) => {
                    self.cmp(&PacketItem::List(vec![PacketItem::Integer(*right)]))
                }
                Self::List(right) => {
                    let zip_result =
                        left.iter()
                            .zip(right.iter())
                            .fold(Ordering::Equal, |acc, (lval, rval)| {
                                if acc != Ordering::Equal {
                                    acc
                                } else {
                                    lval.cmp(rval)
                                }
                            });
                    if zip_result == Ordering::Equal {
                        left.len().cmp(&right.len())
                    } else {
                        zip_result
                    }
                }
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Empty, _) => Ordering::Less,
            (_, Packet::Empty) => Ordering::Greater,
            (Packet::Data(left), Packet::Data(right)) => {
                let mut result = Ordering::Equal;

                for (lval, rval) in left.iter().zip(right.iter()) {
                    result = lval.cmp(rval);

                    if result != Ordering::Equal {
                        break;
                    }
                }

                if result == Ordering::Equal {
                    left.len().cmp(&right.len())
                } else {
                    result
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Empty => match other {
                Self::Empty => true,
                _ => false,
            },
            Self::Data(left) => match other {
                Self::Data(right) => {
                    let mut result = true;

                    for (lval, rval) in left.iter().zip(right.iter()) {
                        result = lval.eq(rval);

                        if !result {
                            break;
                        }
                    }

                    if !result {
                        result
                    } else {
                        left.len().eq(&right.len())
                    }
                }
                _ => false,
            },
        }
    }
}

impl Display for PacketItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().replace("],", "]"))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PacketItem {
    fn new() -> Self {
        PacketItem::List(Vec::new())
    }

    fn new_list(list: Vec<Self>) -> PacketItem {
        PacketItem::List(list)
    }

    fn new_integer(i: u32) -> Self {
        PacketItem::Integer(i)
    }

    fn push(&mut self, new_item: PacketItem) {
        match self {
            Self::Integer(_) => panic!("Cannot push to an Integer."),
            Self::List(list) => list.push(new_item),
        }
    }

    fn to_string(&self) -> String {
        let result = match self {
            PacketItem::Integer(i) => format!("{},", i.to_string()),
            PacketItem::List(list) => format!(
                "[{}],",
                list.iter().fold(String::from(""), |acc, item| format!(
                    "{}{},",
                    acc,
                    item.to_string()
                ))
            ),
        };

        result
    }
}

impl Packet {
    fn empty() -> Self {
        Packet::Empty
    }

    fn new() -> Self {
        Packet::Data(Vec::new())
    }

    fn push(&mut self, new_item: PacketItem) {
        if let Packet::Data(list) = self {
            list.push(new_item);
        }
    }

    fn is_ordered(&self, other: &Self) -> bool {
        match self {
            Packet::Empty => true,
            Packet::Data(left) => match other {
                Packet::Empty => false,
                Packet::Data(right) => {
                    let mut result = true;
                    for (lval, rval) in left.iter().zip(right.iter()) {
                        if lval > rval {
                            result = false;
                            break;
                        }
                    }

                    result
                }
            },
        }
    }

    fn to_string(&self) -> String {
        let result = match self {
            Packet::Empty => format!("[]"),
            Packet::Data(list) => format!(
                "{}",
                list.iter().fold(String::from(""), |acc, item| format!(
                    "{}{},",
                    acc,
                    item.to_string()
                ))
            ),
        };

        String::from(
            result
                .replace(",,", ",")
                .replace(",]", "]")
                .trim_end_matches(","),
        )
    }
}

fn build_packet_list(s: &str) -> Vec<PacketItem> {
    let s = s.strip_prefix('[').unwrap_or(s);
    let s = s.strip_suffix(']').unwrap_or(s);
    let mut list: Vec<PacketItem> = Vec::new();

    let mut current_index = 0;

    loop {
        if current_index >= s.len() {
            break;
        }

        if s.get(current_index..current_index + 1) == Some("[") {
            let bracket_search_index = current_index + 1;
            let mut unmatched_brackets = 1;
            for (ch_index, ch) in s[bracket_search_index..].chars().enumerate() {
                if '[' == ch {
                    unmatched_brackets += 1;
                    continue;
                }

                if ']' == ch {
                    unmatched_brackets -= 1;
                    if unmatched_brackets == 0 {
                        list.push(PacketItem::new_list(build_packet_list(
                            &s[current_index..((bracket_search_index + ch_index) + 1)],
                        )));

                        current_index = (bracket_search_index + ch_index) + 1;
                        break;
                    }
                }
            }
        } else {
            if let Some(i) = &s[current_index..].find(',') {
                list.push(PacketItem::new_integer(
                    u32::from_str(&s[current_index..(current_index + *i)]).unwrap(),
                ));
                current_index += *i;
            } else {
                list.push(PacketItem::new_integer(
                    u32::from_str(&s[current_index..]).unwrap(),
                ));
                current_index = s.len() - 1;
            }
        }

        current_index += 1;
    }

    list
}

fn get_data() -> String {
    let input = match fs::read_to_string("input") {
        Ok(data) => data,
        Err(error) => panic!("Error opening input file: {:?}", error),
    };

    input
}

fn part_one() {
    let mut pairs: Vec<(u32, (Packet, Packet))> = Vec::new();
    let mut current_pair: (Option<Packet>, Option<Packet>) = (None, None);

    let mut current_index = 1;

    for line in get_data().lines() {
        if line.is_empty() {
            pairs.push((
                current_index,
                (
                    match &current_pair.0 {
                        Some(packet) => (*packet).clone(),
                        None => panic!(""),
                    },
                    match &current_pair.1 {
                        Some(packet) => (*packet).clone(),
                        None => panic!(""),
                    },
                ),
            ));

            current_index += 1;
            current_pair.0 = None;
            current_pair.1 = None;
            continue;
        }

        let mut p = Packet::new();
        p.push(PacketItem::List(build_packet_list(line)));

        if let None = current_pair.0 {
            current_pair.0 = Some(p);
        } else {
            current_pair.1 = Some(p)
        }
    }

    pairs.push((
        current_index,
        (
            match &current_pair.0 {
                Some(packet) => (*packet).clone(),
                None => panic!(""),
            },
            match &current_pair.1 {
                Some(packet) => (*packet).clone(),
                None => panic!(""),
            },
        ),
    ));

    let mut sum = 0;

    for pair in pairs {
        print!("Comparing {} and {}...", &pair.1 .0, &pair.1 .1);
        if pair.1 .0.is_ordered(&pair.1 .1) {
            println!("they are ordered.");
            sum += pair.0;
        } else {
            println!("they are not ordered.");
        }
    }

    println!("sum: {}", sum);
}

fn part_two() {
    let mut packets: Vec<Packet> = Vec::new();

    for line in get_data().lines() {
        if line.is_empty() {
            continue;
        }
        let mut p = Packet::new();
        p.push(PacketItem::List(build_packet_list(line)));

        packets.push(p);
    }

    let mut p = Packet::new();
    p.push(PacketItem::List(build_packet_list("[[2]]")));
    packets.push(p);

    p = Packet::new();
    p.push(PacketItem::List(build_packet_list("[[6]]")));
    packets.push(p);

    packets.sort();
    println!(
        "{}",
        packets
            .iter()
            .fold(String::from(""), |acc, p| format!("{}{}\n", acc, p))
    );

    let key = packets.iter().enumerate().fold(1, |acc, (index, p)| {
        if p.to_string() == "[[2]]" || p.to_string() == "[[6]]" {
            (index + 1) * acc
        } else {
            acc
        }
    });

    println!("key: {}", key);
}

fn main() {
    part_two();
}
