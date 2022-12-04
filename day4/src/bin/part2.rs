use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let sections = contents.lines().collect::<Vec<&str>>();

    println!("overlap pairs {}", calculate_section_overlap(sections));
}

fn calculate_section_overlap(sections: Vec<&str>) -> u32 {
    let mut overlaps = 0;

    for s in sections {
        let section_block: Vec<&str> = s.split(",").collect();
        let block_one = section_block.first().unwrap();
        let block_two = section_block.last().unwrap();

        let block_one_data: Vec<u32> = block_one
            .split("-")
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let block_two_data: Vec<u32> = block_two
            .split("-")
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if block_one_data[0] <= block_two_data[0] && block_two_data[0] <= block_one_data[1] {
            overlaps += 1;
        } else if block_one_data[0] <= block_two_data[1] && block_two_data[1] <= block_one_data[0] {
            overlaps += 1;
        } else if block_two_data[0] <= block_one_data[0] && block_one_data[0] <= block_two_data[1] {
            overlaps += 1;
        } else if block_two_data[0] <= block_one_data[1] && block_one_data[1] <= block_two_data[0] {
            overlaps += 1;
        }
    }

    overlaps
}
