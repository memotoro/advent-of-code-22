use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let input = contents.lines().collect::<Vec<&str>>();

    println!("size to free {}", calculate_space(input));
}

fn calculate_space(input: Vec<&str>) -> u32 {
    let mut dirs_size: HashMap<String, u32> = HashMap::new();
    let mut dirs: Vec<String> = Vec::new();
    let user_command_token = "$ ";
    let empty_option = Some(&"");

    for i in input {
        if i.contains(user_command_token) {
            let user_command = i.replace(user_command_token, "");
            let user_command_data = user_command.split(" ").collect::<Vec<&str>>();
            let command = user_command_data.first().unwrap();
            let argument = user_command_data.last().or(empty_option).unwrap();

            if *command == "cd" {
                if *argument == ".." {
                    add_to_parent(&mut dirs, &mut dirs_size);
                } else {
                    let arg = String::from(*argument);
                    going_into_dir(&mut dirs, &mut dirs_size, arg);
                }
            }
        } else {
            let system_output_data = i.split(" ").collect::<Vec<&str>>();
            let output = system_output_data.first().unwrap();

            if !output.contains("dir") {
                sum_files(output, &dirs, &mut dirs_size);
            }
        }
    }

    // Adding last calculation if the stack is not in the root
    while dirs.len() > 1 {
        add_to_parent(&mut dirs, &mut dirs_size);
    }

    let mut max_size = 0;

    for (_k, v) in dirs_size.clone() {
        if v >= max_size {
            max_size = v;
        }
    }

    let max_file_size = 70000000;
    let min_free_space = 30000000;
    let unused = max_file_size - max_size;

    let size_to_free = min_free_space - unused;
    let mut selected_size = max_size;

    for (_k, v) in dirs_size {
        if v >= size_to_free && v <= selected_size {
            selected_size = v
        }
    }

    selected_size
}

fn add_to_parent(dirs: &mut Vec<String>, dirs_size: &mut HashMap<String, u32>) {
    let mut dir = dirs.last().unwrap();
    let size = dirs_size.get(dir).unwrap();
    dirs.pop().unwrap();

    dir = dirs.last().unwrap();
    let current_size = dirs_size.get(dir).unwrap();
    dirs_size.insert(dir.to_string(), current_size + size);
}

fn going_into_dir(dirs: &mut Vec<String>, dirs_size: &mut HashMap<String, u32>, argument: String) {
    let mut canonical_dir = argument.clone();
    if let Some(dir) = dirs.last() {
        canonical_dir = format!("{}/{}", dir, argument);
    }

    dirs.push(canonical_dir.clone());
    dirs_size.insert(canonical_dir, 0);
}

fn sum_files(output: &str, dirs: &Vec<String>, dirs_size: &mut HashMap<String, u32>) {
    let file_size = output.trim().parse::<u32>().unwrap();
    let dir = dirs.last().unwrap();
    let size = dirs_size.get(dir).unwrap();
    dirs_size.insert(dir.to_string(), file_size + size);
}
