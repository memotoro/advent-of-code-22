use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let contents = contents.lines().collect::<Vec<&str>>();

    println!("priority {}", calculate_priority(contents));
}

fn calculate_priority(contents: Vec<&str>) -> u32 {
    let mut priority: u32 = 0;

    for content in contents {
        let mid = content.len() / 2;
        let first_item = content.get(0..mid).unwrap();
        let second_item = content.get(mid..content.len()).unwrap();

        let letters: Vec<char> = first_item.chars().collect();
        let mut item_type: u32 = 0;

        for letter in second_item.chars() {
            if letters.contains(&letter) {
                item_type = letter as u32;
                break;
            }
        }

        priority += convert_ascii_priority(item_type);
    }

    priority
}

fn convert_ascii_priority(number: u32) -> u32 {
    let mut priority: u32 = 0;

    if number >= 65 && number <= 90 {
        priority = number - 64 + 26
    } else if number >= 97 && number <= 122 {
        priority = number - 96;
    }

    priority
}
