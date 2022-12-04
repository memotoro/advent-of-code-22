use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let contents = contents.lines().collect::<Vec<&str>>();

    println!("priority {}", calculate_priority(contents));
}

fn calculate_priority(contents: Vec<&str>) -> u32 {
    let mut priority: u32 = 0;
    let mut counter = 1;

    while counter <= contents.len() {
        if counter % 3 == 0 {
            let mut item_type: u32 = 0;

            let first_items: Vec<char> = contents[counter - 3].chars().collect();

            let mut second_items: Vec<char> = Vec::new();

            for letter in contents[counter - 2].chars() {
                if first_items.contains(&letter) {
                    second_items.push(letter);
                }
            }

            for letter in contents[counter - 1].chars() {
                if second_items.contains(&letter) {
                    item_type = letter as u32;
                    break;
                }
            }

            priority += convert_ascii_priority(item_type);
        }

        counter += 1;
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
