use std::fs;
use std::str::FromStr;

struct File {
    size: u64,
    name: String,
}

impl File {
    fn new(size: u64, name: String) -> Self {
        File { size, name }
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

struct Directory {
    parent: Option<usize>,
    children: Vec<usize>,
    files: Vec<usize>,
    name: String,
}

impl Directory {
    fn new(parent: Option<usize>, name: String) -> Self {
        Self {
            parent,
            children: Vec::new(),
            files: Vec::new(),
            name,
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self, all_files: &Vec<File>, all_directories: &Vec<Self>) -> u64 {
        let mut total_size = 0;

        for file in &self.files {
            total_size += all_files[*file].size();
        }

        for directory in &self.children {
            total_size += all_directories[*directory].size(all_files, all_directories);
        }

        total_size
    }

    fn get_child_by_name(&self, child_name: &str, all_directories: &Vec<Self>) -> Option<usize> {
        for child in &self.children {
            if all_directories[*child].name == child_name {
                return Some(*child);
            }
        }

        None
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
    let mut directories: Vec<Directory> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut complete: bool = false;

    let root_directory: Directory = Directory::new(None, String::from("/"));

    directories.push(root_directory);

    let mut current_directory: usize = 0;

    let input = get_data();

    let mut lines = input.lines().peekable();

    while !complete {
        let next_line = lines.next();

        if next_line == None {
            complete = true;
            continue;
        }

        let next_line_value = next_line.unwrap();

        if next_line_value.starts_with("$") {
            let mut command_parts = next_line_value.split_whitespace();
            command_parts.next();
            let command = command_parts.next().unwrap();

            if command == "ls" {
                loop {
                    if lines.peek().unwrap_or(&"$").starts_with("$") {
                        break;
                    }

                    let list_item = lines.next().unwrap();
                    let mut list_item_parts = list_item.split_whitespace();

                    let first_part = list_item_parts.next().unwrap();

                    if first_part == "dir" {
                        let new_directory_name = list_item_parts.next().unwrap();
                        directories.push(Directory::new(
                            Some(current_directory),
                            String::from(new_directory_name),
                        ));

                        let new_index = directories.len() - 1;

                        directories[current_directory].children.push(new_index);
                    } else {
                        let filename = list_item_parts.next().unwrap();
                        files.push(File::new(
                            u64::from_str(first_part).unwrap(),
                            String::from(filename),
                        ));

                        directories[current_directory].files.push(files.len() - 1);
                    }
                }

                continue;
            }

            if command == "cd" {
                let command_arg = command_parts.next().unwrap();

                if command_arg == "/" {
                    current_directory = 0;
                } else if command_arg == ".." {
                    current_directory = match directories[current_directory].parent {
                        Some(s) => s,
                        None => 0,
                    }
                } else {
                    let index =
                        directories[current_directory].get_child_by_name(command_arg, &directories);

                    current_directory = match index {
                        Some(i) => i,
                        None => panic!("Cannot cd to {}", command_arg),
                    }
                }
            }
        }
    }

    let mut total_size = 0;

    for dir in &directories {
        let current_size = dir.size(&files, &directories);

        if current_size < 100000 {
            total_size += current_size;
        }
    }

    println!("total size is {}", total_size);
}

fn part_two() {
    let mut directories: Vec<Directory> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut complete: bool = false;

    let root_directory: Directory = Directory::new(None, String::from("/"));

    directories.push(root_directory);

    let mut current_directory: usize = 0;

    let input = get_data();

    let mut lines = input.lines().peekable();

    while !complete {
        let next_line = lines.next();

        if next_line == None {
            complete = true;
            continue;
        }

        let next_line_value = next_line.unwrap();

        if next_line_value.starts_with("$") {
            let mut command_parts = next_line_value.split_whitespace();
            command_parts.next();
            let command = command_parts.next().unwrap();

            if command == "ls" {
                loop {
                    if lines.peek().unwrap_or(&"$").starts_with("$") {
                        break;
                    }

                    let list_item = lines.next().unwrap();
                    let mut list_item_parts = list_item.split_whitespace();

                    let first_part = list_item_parts.next().unwrap();

                    if first_part == "dir" {
                        let new_directory_name = list_item_parts.next().unwrap();
                        directories.push(Directory::new(
                            Some(current_directory),
                            String::from(new_directory_name),
                        ));

                        let new_index = directories.len() - 1;

                        directories[current_directory].children.push(new_index);
                    } else {
                        let filename = list_item_parts.next().unwrap();
                        files.push(File::new(
                            u64::from_str(first_part).unwrap(),
                            String::from(filename),
                        ));

                        directories[current_directory].files.push(files.len() - 1);
                    }
                }

                continue;
            }

            if command == "cd" {
                let command_arg = command_parts.next().unwrap();

                if command_arg == "/" {
                    current_directory = 0;
                } else if command_arg == ".." {
                    current_directory = match directories[current_directory].parent {
                        Some(s) => s,
                        None => 0,
                    }
                } else {
                    let index =
                        directories[current_directory].get_child_by_name(command_arg, &directories);

                    current_directory = match index {
                        Some(i) => i,
                        None => panic!("Cannot cd to {}", command_arg),
                    }
                }
            }
        }
    }

    let root_size = directories[0].size(&files, &directories);
    let needed_space = 30000000 - (70000000 - root_size);
    let mut smallest_size = root_size;

    for dir in &directories {
        let current_size = dir.size(&files, &directories);

        if current_size >= needed_space && current_size < smallest_size {
            smallest_size = current_size;
        }
    }

    println!("smallest sufficient size is {}", smallest_size);
}

fn main() {
    // part_one();
    part_two();
}
