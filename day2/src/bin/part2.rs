use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("PART2_INPUT_FILE").expect("error getting env variable");

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
     * X: Lose
     * Y: Draw
     * Z: Win
     */
    let mut item_scores: HashMap<&str, u32> = HashMap::new();
    item_scores.insert("A", 1);
    item_scores.insert("B", 2);
    item_scores.insert("C", 3);

    let mut win_plays: HashMap<&str, &str> = HashMap::new();
    win_plays.insert("A", "B");
    win_plays.insert("B", "C");
    win_plays.insert("C", "A");

    let mut lose_plays: HashMap<&str, &str> = HashMap::new();
    lose_plays.insert("A", "C");
    lose_plays.insert("B", "A");
    lose_plays.insert("C", "B");

    let mut score = 0;

    for s in strategies {
        let plays: Vec<&str> = s.split(" ").collect();
        let opponent_play = plays.first().unwrap();
        let my_play = plays.last().unwrap();
        let item: &str;

        if *my_play == "X" {
            item = lose_plays.get(*opponent_play).unwrap();
        } else if *my_play == "Z" {
            item = win_plays.get(*opponent_play).unwrap();
            score += 6;
        } else {
            item = *opponent_play;
            score += 3;
        }

        score += item_scores.get(item).unwrap();
    }

    score
}
