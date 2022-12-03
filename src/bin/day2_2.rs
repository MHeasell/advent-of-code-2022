
use std::{fs::File, io::{BufReader, BufRead}};

const OUTCOME_LOSE: char = 'X';
const OUTCOME_DRAW: char = 'Y'; 
const OUTCOME_WIN: char = 'Z'; 

const ME_ROCK: char = 'X';
const ME_PAPER: char = 'Y'; 
const ME_SCISSORS: char = 'Z'; 

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B'; 
const OPPONENT_SCISSORS: char = 'C'; 

fn main() {
    let file = File::open("data/day2/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();


    let total = lines.into_iter().map(|l| parse_and_score(&l.unwrap())).sum::<i32>();

    println!("Total: {}", total);
}

fn parse_and_score(line: &str) -> i32 {
    let opponent_choice = line.chars().nth(0).unwrap();
    let desired_outcome = line.chars().nth(2).unwrap();
    score_round(opponent_choice, desired_outcome)
}

fn score_shape(my_choice: char) -> i32 {
    match my_choice {
        ME_ROCK => 1,
        ME_PAPER => 2,
        ME_SCISSORS => 3,
         _ => panic!("invalid choice {}", my_choice),
    }
}

const SCORE_LOSE: i32 = 0;
const SCORE_DRAW: i32 = 3;
const SCORE_WIN: i32 = 6;

fn score_outcome(opponent_choice: char, my_choice: char) -> i32 {
    match (opponent_choice, my_choice) {
        (OPPONENT_ROCK, ME_ROCK) => SCORE_DRAW,
        (OPPONENT_ROCK, ME_PAPER) => SCORE_WIN,
        (OPPONENT_ROCK, ME_SCISSORS) => SCORE_LOSE,

        (OPPONENT_PAPER, ME_ROCK) => SCORE_LOSE,
        (OPPONENT_PAPER, ME_PAPER) => SCORE_DRAW,
        (OPPONENT_PAPER, ME_SCISSORS) => SCORE_WIN,

        (OPPONENT_SCISSORS, ME_ROCK) => SCORE_WIN,
        (OPPONENT_SCISSORS, ME_PAPER) => SCORE_LOSE,
        (OPPONENT_SCISSORS, ME_SCISSORS) => SCORE_DRAW,

         _ => panic!("invalid combo '{} {}'", opponent_choice, my_choice),
    }
}

fn get_my_choice_for_outcome(opponent_choice: char, desired_outcome: char) -> char {
    match(opponent_choice, desired_outcome) {
        (OPPONENT_ROCK, OUTCOME_LOSE) => ME_SCISSORS,
        (OPPONENT_ROCK, OUTCOME_DRAW) => ME_ROCK,
        (OPPONENT_ROCK, OUTCOME_WIN) => ME_PAPER,

        (OPPONENT_PAPER, OUTCOME_LOSE) => ME_ROCK,
        (OPPONENT_PAPER, OUTCOME_DRAW) => ME_PAPER,
        (OPPONENT_PAPER, OUTCOME_WIN) => ME_SCISSORS,

        (OPPONENT_SCISSORS, OUTCOME_LOSE) => ME_PAPER,
        (OPPONENT_SCISSORS, OUTCOME_DRAW) => ME_SCISSORS,
        (OPPONENT_SCISSORS, OUTCOME_WIN) => ME_ROCK,

         _ => panic!("invalid combo '{} {}'", opponent_choice, desired_outcome),
    }
}

fn score_round(opponent_choice: char, desired_outcome: char) -> i32 {
    let my_choice = get_my_choice_for_outcome(opponent_choice, desired_outcome);
    let shape_score = score_shape(my_choice);
    let round_score = score_outcome(opponent_choice, my_choice);
    shape_score + round_score
}
