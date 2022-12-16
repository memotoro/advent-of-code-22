use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let input = contents.lines().collect::<Vec<&str>>();

    println!("visible trees {}", calculate_visible_trees(input));
}

fn calculate_visible_trees(input: Vec<&str>) -> u32 {
    let forest = build_forest(input);
    let cols_limit = forest.first().unwrap().len();
    let rows_limit = forest.len();

    let mut visible_trees = ((cols_limit * 2) + (rows_limit * 2 - 4)) as u32;

    for r in 1..(rows_limit - 1) {
        for c in 1..(cols_limit - 1) {
            let checked_tree = forest[r][c];
            let mut not_visible = true;

            // Check for left elements
            let mut k = 0;
            while k < c {
                not_visible = checked_tree <= forest[r][k];
                if not_visible {
                    break;
                }
                k += 1;
            }

            if !not_visible {
                visible_trees = check_visible_trees(not_visible, visible_trees);
                continue;
            }

            // Check for right elements
            k = cols_limit - 1;
            while k > c {
                not_visible = checked_tree <= forest[r][k];
                if not_visible {
                    break;
                }
                k -= 1;
            }

            if !not_visible {
                visible_trees = check_visible_trees(not_visible, visible_trees);
                continue;
            }

            // Check for top elements
            k = 0;
            while k < r {
                not_visible = checked_tree <= forest[k][c];
                if not_visible {
                    break;
                }
                k += 1;
            }

            if !not_visible {
                visible_trees = check_visible_trees(not_visible, visible_trees);
                continue;
            }

            // Check for down elements
            k = rows_limit - 1;
            while k > r {
                not_visible = checked_tree <= forest[k][c];
                if not_visible {
                    break;
                }
                k -= 1;
            }

            if !not_visible {
                visible_trees = check_visible_trees(not_visible, visible_trees);
                continue;
            }
        }
    }

    visible_trees
}

fn check_visible_trees(not_visible: bool, visible_trees: u32) -> u32 {
    let mut value = visible_trees;
    if !not_visible {
        value = visible_trees + 1;
    }

    value
}

fn build_forest(input: Vec<&str>) -> Vec<Vec<u32>> {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let cols_limit = input.first().unwrap().chars().collect::<Vec<char>>().len();
    let rows_limit = input.len();

    for i in 0..cols_limit {
        for j in 0..rows_limit {
            let data_row = input.get(i).unwrap();

            let data_chars = data_row.chars().collect::<Vec<char>>();

            let raw_data = data_chars.get(j).unwrap();

            let height = raw_data.to_digit(10).unwrap();

            if forest.get(i).is_none() {
                forest.push(Vec::new());
            }

            let row = forest.get_mut(i).unwrap();
            row.push(height);
        }
    }

    forest
}
