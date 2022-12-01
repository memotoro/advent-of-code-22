use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let calories = contents.lines().collect::<Vec<&str>>();

    println!("calories {}", calculate_max_calories(calories));
}

fn calculate_max_calories(calories: Vec<&str>) -> u32 {
    let mut elf_calories = 0;
    let mut elf_total_calories: Vec<u32> = Vec::new();

    for c in calories {
        if c == "" {
            elf_total_calories.push(elf_calories);
            elf_calories = 0;
            continue;
        }

        let calorie = c.parse::<u32>().unwrap();
        elf_calories += calorie;
    }

    // Add last element read
    elf_total_calories.push(elf_calories);

    elf_total_calories.sort_by(|a, b| b.cmp(a));

    let mut total_calories = 0;
    for i in 0..3 {
        total_calories += elf_total_calories[i];
    }

    total_calories
}
