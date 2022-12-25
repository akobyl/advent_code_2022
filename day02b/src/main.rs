const ROUND_LOSE: &str = "X";
const ROUND_TIE: &str = "Y";
const ROUND_WIN: &str = "Z";
const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const SCORE_WIN: u32 = 6;
const SCORE_TIE: u32 = 3;
const SCORE_LOSE: u32 = 0;
const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;


pub fn main() {
    let x: u32 = include_str!("../../day02a/input.txt")
        .lines()
        .map(|s| -> u32 { calc_score(s) })
        .sum::<u32>();
    println!("{x}");
}

fn outcome_points(goal: &str, opponent: &str) -> u32 {
    let mut score;
    match goal {
        ROUND_LOSE => {
            score = SCORE_LOSE;
            match opponent {
                OPPONENT_PAPER => score += SCORE_ROCK,
                OPPONENT_ROCK => score += SCORE_SCISSORS,
                OPPONENT_SCISSORS => score += SCORE_PAPER,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        ROUND_TIE => {
            score = SCORE_TIE;
            match opponent {
                OPPONENT_PAPER => score += SCORE_PAPER,
                OPPONENT_ROCK => score += SCORE_ROCK,
                OPPONENT_SCISSORS => score += SCORE_SCISSORS,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        ROUND_WIN => {
            score = SCORE_WIN;
            match opponent {
                OPPONENT_PAPER => score += SCORE_SCISSORS,
                OPPONENT_ROCK => score += SCORE_PAPER,
                OPPONENT_SCISSORS => score += SCORE_ROCK,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        _ => panic!("invalid mine option {goal}"),
    }

    return score;
}

fn calc_score(input: &str) -> u32 {
    let plays: Vec<&str> = input.split(' ').collect();
    return outcome_points(plays[1], plays[0]);
}
