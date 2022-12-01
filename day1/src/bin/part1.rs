use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let calories = contents.lines().collect::<Vec<&str>>();

    println!("calories {}", calculate_max_calories(calories));
}

fn calculate_max_calories(calories: Vec<&str>) -> u32 {
    let mut max_calories = 0;
    let mut elf_calories = 0;

    for c in calories {
        if c == "" {
            max_calories = max_value(max_calories, elf_calories);
            elf_calories = 0;
            continue;
        }

        let calorie = c.parse::<u32>().unwrap();
        elf_calories += calorie
    }

    max_value(max_calories, elf_calories)
}

fn max_value(max: u32, current: u32) -> u32 {
    let mut value = max;
    if max < current {
        value = current;
    }

    value
}
