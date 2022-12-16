use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let input = contents.lines().collect::<Vec<&str>>();

    println!("scenic score {}", calculate_scenic_score(input));
}

fn calculate_scenic_score(input: Vec<&str>) -> u32 {
    let forest = build_forest(input);
    let cols_limit = forest.first().unwrap().len();
    let rows_limit = forest.len();

    let mut scenic_score = 0;

    for r in 1..(rows_limit - 1) {
        for c in 1..(cols_limit - 1) {
            let checked_tree = forest[r][c];
            let mut score = 1;

            // Check for left elements
            if c > 0 {
                let mut k: i32 = (c - 1) as i32;
                let mut local_score = 0;

                while k >= 0 {
                    let moving_tree = forest[r][k as usize];

                    local_score += 1;
                    if checked_tree <= moving_tree {
                        break;
                    }
                    k -= 1;
                }

                score *= local_score;
            }

            // Check for right elements
            if c < cols_limit - 1 {
                let mut k = c + 1;
                let mut local_score = 0;

                while k < cols_limit {
                    let moving_tree = forest[r][k];

                    local_score += 1;
                    if checked_tree <= moving_tree {
                        break;
                    }
                    k += 1;
                }

                score *= local_score;
            }

            // Check for top elements
            if r > 0 {
                let mut k: i32 = (r - 1) as i32;
                let mut local_score = 0;

                while k >= 0 {
                    let moving_tree = forest[k as usize][c];

                    local_score += 1;
                    if checked_tree <= moving_tree {
                        break;
                    }
                    k -= 1;
                }

                score *= local_score;
            }

            // Check for down elements
            if r < rows_limit - 1 {
                let mut k = r + 1;
                let mut local_score = 0;

                while k < rows_limit {
                    let moving_tree = forest[k][c];

                    local_score += 1;
                    if checked_tree <= moving_tree {
                        break;
                    }
                    k += 1;
                }

                score *= local_score;
            }

            if scenic_score < score {
                scenic_score = score;
            }
        }
    }

    scenic_score
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
