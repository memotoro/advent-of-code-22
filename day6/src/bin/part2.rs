use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let datastreams = contents.lines().collect::<Vec<&str>>();

    println!("last marker {}", calculate_marker(datastreams));
}

fn calculate_marker(datastreams: Vec<&str>) -> u32 {
    let mut marker: u32 = 0;
    let message_size = 14;

    for ds in datastreams {
        marker = read_data(ds, message_size);
    }

    marker
}

fn read_data(ds: &str, size: usize) -> u32 {
    let zero = Some(&0);
    let mut marker: u32 = 0;
    let mut lookup: HashMap<&char, u32> = HashMap::new();
    let datastream = ds.chars().collect::<Vec<char>>();

    for (index, d) in datastream.iter().enumerate() {
        lookup.insert(d, lookup.get(d).or(zero).unwrap() + 1);

        if index < size - 1 {
            continue;
        }

        if lookup.len() < size {
            let last_char = &datastream[index - (size - 1)];
            let value = lookup.get(last_char).or(zero).unwrap();
            if *value == 1 {
                lookup.remove(last_char);
            } else {
                lookup.insert(last_char, lookup.get(last_char).or(zero).unwrap() - 1);
            }
        } else {
            marker = (index as u32) + 1;
            println!("data {:?} marker {}", ds, marker);
            break;
        }
    }

    marker
}
