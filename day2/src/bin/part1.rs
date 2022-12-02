use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("something went wrong reading the file");

    let strategies = contents.lines().collect::<Vec<&str>>();

    println!("score {}", calculate_score(strategies));
}

fn calculate_score(strategies: Vec<&str>) -> u32 {
    /*
     * Opponent
     * A: Rock
     * B: Paper
     * C: Scissors
     *
     * Me
     * X: Rock
     * Y: Paper
     * Z: Scissors
     */
    let mut item_scores: HashMap<&str, u32> = HashMap::new();
    item_scores.insert("X", 1);
    item_scores.insert("Y", 2);
    item_scores.insert("Z", 3);

    let mut scoring_plays: HashMap<&str, u32> = HashMap::new();
    scoring_plays.insert("A Y", 6);
    scoring_plays.insert("B Z", 6);
    scoring_plays.insert("C X", 6);
    scoring_plays.insert("A X", 3);
    scoring_plays.insert("B Y", 3);
    scoring_plays.insert("C Z", 3);

    let mut score = 0;

    for s in strategies {
        let plays: Vec<&str> = s.split(" ").collect();
        let my_play = plays.last().unwrap();

        score += item_scores.get(my_play).unwrap();

        if scoring_plays.contains_key(s) {
            score += scoring_plays.get(s).unwrap();
        }
    }

    score
}
