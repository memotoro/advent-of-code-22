use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let input = contents.lines().collect::<Vec<&str>>();

    println!("top crates {}", calculate_stacks(input));
}

fn calculate_stacks(input: Vec<&str>) -> String {
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut stack_number_lines = 0;

    for (i, v) in input.iter().enumerate() {
        // Find fist line with numbers
        if v.contains("1") {
            stack_number_lines = i;
            break;
        }
    }

    // Maximum 9 crates
    for i in 1..10 {
        let number_line = input.get(stack_number_lines).unwrap();
        let index = match number_line.find(i.to_string().as_str()) {
            Some(i) => i,
            None => break,
        };

        let mut stack: Vec<String> = Vec::new();
        let mut read_line: i32 = (stack_number_lines - 1) as i32;

        while read_line >= 0 {
            let line = input.get(read_line as usize).unwrap();
            let character = line.chars().nth(index).unwrap();
            let character_string = character.to_string();
            let character_data = character_string.trim();

            if !character_data.is_empty() {
                stack.push(character_data.to_string());
            }

            read_line -= 1;
        }

        stacks.push(stack);
    }

    print_stacks(&stacks);

    for i in input {
        if !i.contains("move") {
            continue;
        }

        let move_word = i.find("move").unwrap();
        let from_word = i.find("from").unwrap();
        let to_word = i.find("to").unwrap();

        let quantity = i
            .get((move_word + 4)..from_word) // 4 chars of word move
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        let stack_from_index = i
            .get((from_word + 4)..to_word) // 4 chars of word from
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap()
            - 1;

        let stack_to_index = i
            .get((to_word + 2)..i.len()) // 2 chars of word to
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap()
            - 1;

        let mut q = 0;
        while q < quantity {
            let stack_from = stacks.get_mut(stack_from_index as usize).unwrap();
            let value = &stack_from.pop().unwrap();

            let stack_to = stacks.get_mut(stack_to_index as usize).unwrap();
            stack_to.push(value.to_string());
            q += 1;
        }

        print_stacks(&stacks);
    }

    let mut crates = "".to_string();

    for mut s in stacks {
        crates.push_str(s.pop().unwrap().as_str());
    }

    crates
}

fn print_stacks(stacks: &Vec<Vec<String>>) {
    for (i, s) in stacks.iter().enumerate() {
        println!("{} stack {:?}", i + 1, s);
    }
    println!("==========================");
}
