const MINE_ROCK: &str = "X";
const MINE_PAPER: &str = "Y";
const MINE_SCISSORS: &str = "Z";
const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const SCORE_WIN: u32 = 6;
const SCORE_TIE: u32 = 3;
const SCORE_LOSE: u32 = 0;


pub fn main() {
    let x: u32 = include_str!("../input.txt")
        .lines()
        .map(|s| -> u32 { calc_score(s) })
        .sum::<u32>();
    println!("{x}");
}

fn outcome_points(mine: &str, opponent: &str) -> u32 {
    match mine {
        MINE_PAPER => {
            match opponent {
                OPPONENT_PAPER =>  return SCORE_TIE,
                OPPONENT_ROCK =>  return SCORE_WIN,
                OPPONENT_SCISSORS => return SCORE_LOSE,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        MINE_ROCK => {
            match opponent {
                OPPONENT_PAPER =>  return SCORE_LOSE,
                OPPONENT_ROCK =>  return SCORE_TIE,
                OPPONENT_SCISSORS =>  return SCORE_WIN,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        MINE_SCISSORS => {
            match opponent {
                OPPONENT_PAPER =>  return SCORE_WIN,
                OPPONENT_ROCK =>  return SCORE_LOSE,
                OPPONENT_SCISSORS =>  return SCORE_TIE,
                _ => panic!("invalid opponent option {opponent}"),
            }
        }
        _ => panic!("invalid mine option {mine}"),
    }
}

fn calc_score(input: &str) -> u32 {
    let plays: Vec<&str> = input.split(' ').collect();
    let mut score = outcome_points(plays[1], plays[0]);

    match plays[1] {
        MINE_ROCK => {
            score += 1;
        }
        MINE_PAPER=> {
            score += 2;
        }
        MINE_SCISSORS => {
            score += 3;
        }
        _ => panic!("invalid option {}", plays[1]),
    }

    return score;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcomes() {
        assert_eq!(outcome_points(MINE_PAPER, OPPONENT_ROCK), SCORE_WIN);
        assert_eq!(outcome_points(MINE_PAPER, OPPONENT_SCISSORS), SCORE_LOSE);
        assert_eq!(outcome_points(MINE_PAPER, OPPONENT_PAPER), SCORE_TIE);

        assert_eq!(outcome_points(MINE_SCISSORS, OPPONENT_ROCK), SCORE_LOSE);
        assert_eq!(outcome_points(MINE_SCISSORS, OPPONENT_SCISSORS), SCORE_TIE);
        assert_eq!(outcome_points(MINE_SCISSORS, OPPONENT_PAPER), SCORE_WIN);

        assert_eq!(outcome_points(MINE_ROCK, OPPONENT_ROCK), SCORE_TIE);
        assert_eq!(outcome_points(MINE_ROCK, OPPONENT_SCISSORS), SCORE_WIN);
        assert_eq!(outcome_points(MINE_ROCK, OPPONENT_PAPER), SCORE_LOSE);
    }

    #[test]
    fn test_calc_score() {
        assert_eq!(calc_score("A Y"), 8);
        assert_eq!(calc_score("B X"), 1);
        assert_eq!(calc_score("C Z"), 6);
    }
}