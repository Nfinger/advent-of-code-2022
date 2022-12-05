const ROCK_OPPONENT: &str = "A";
const PAPER_OPPONENT: &str = "B";
const SCISSORS_OPPONENT: &str = "C";

const ROCK_PLAYER: &str = "X";
const PAPER_PLAYER: &str = "Y";
const SCISSORS_PLAYER: &str = "Z";

const LOSE_SCENARIO: &str = "X";
const DRAW_SCENARIO: &str = "Y";
const WIN_SCENARIO: &str = "Z";

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const LOSING_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WINNING_SCORE: u32 = 6;

fn calculate_winner_first(player_one: &str, player_two: &str) -> u32 {
    match (player_one, player_two) {
        (ROCK_OPPONENT, SCISSORS_PLAYER) => LOSING_SCORE + SCISSORS_SCORE,
        (PAPER_OPPONENT, ROCK_PLAYER) => LOSING_SCORE + ROCK_SCORE,
        (SCISSORS_OPPONENT, PAPER_PLAYER) => LOSING_SCORE + PAPER_SCORE,
        (ROCK_OPPONENT, PAPER_PLAYER) => WINNING_SCORE + PAPER_SCORE,
        (PAPER_OPPONENT, SCISSORS_PLAYER) => WINNING_SCORE + SCISSORS_SCORE,
        (SCISSORS_OPPONENT, ROCK_PLAYER) => WINNING_SCORE + ROCK_SCORE,
        (ROCK_OPPONENT, ROCK_PLAYER) => DRAW_SCORE + ROCK_SCORE,
        (PAPER_OPPONENT, PAPER_PLAYER) => DRAW_SCORE + PAPER_SCORE,
        (SCISSORS_OPPONENT, SCISSORS_PLAYER) => DRAW_SCORE + SCISSORS_SCORE,
        _ => 0,
    }
}

fn calculate_winner_second(player_one: &str, outcome: &str) -> u32 {
    match (player_one, outcome) {
        (ROCK_OPPONENT, LOSE_SCENARIO) => LOSING_SCORE + SCISSORS_SCORE,
        (PAPER_OPPONENT, LOSE_SCENARIO) => LOSING_SCORE + ROCK_SCORE,
        (SCISSORS_OPPONENT, LOSE_SCENARIO) => LOSING_SCORE + PAPER_SCORE,
        (ROCK_OPPONENT, WIN_SCENARIO) => WINNING_SCORE + PAPER_SCORE,
        (PAPER_OPPONENT, WIN_SCENARIO) => WINNING_SCORE + SCISSORS_SCORE,
        (SCISSORS_OPPONENT, WIN_SCENARIO) => WINNING_SCORE + ROCK_SCORE,
        (ROCK_OPPONENT, DRAW_SCENARIO) => DRAW_SCORE + ROCK_SCORE,
        (PAPER_OPPONENT, DRAW_SCENARIO) => DRAW_SCORE + PAPER_SCORE,
        (SCISSORS_OPPONENT, DRAW_SCENARIO) => DRAW_SCORE + SCISSORS_SCORE,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(|l| {
            let mut player_moves = l.split_whitespace().map(|s| match s {
                "A" => ROCK_OPPONENT,
                "B" => PAPER_OPPONENT,
                "C" => SCISSORS_OPPONENT,
                "X" => ROCK_PLAYER,
                "Y" => PAPER_PLAYER,
                "Z" => SCISSORS_PLAYER,
                _ => "",
            });

            let player_one = player_moves.next().unwrap_or("");
            let player_two = player_moves.next().unwrap_or("");

            if (player_one == "" || player_two == "") {
                return 0;
            }
            calculate_winner_first(player_one, player_two)
        })
        .sum();
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(|l| {
            let mut player_moves = l.split_whitespace().map(|s| match s {
                "A" => ROCK_OPPONENT,
                "B" => PAPER_OPPONENT,
                "C" => SCISSORS_OPPONENT,
                "X" => ROCK_PLAYER,
                "Y" => PAPER_PLAYER,
                "Z" => SCISSORS_PLAYER,
                _ => "",
            });

            let player_one = player_moves.next().unwrap_or("");
            let player_two = player_moves.next().unwrap_or("");

            if (player_one == "" || player_two == "") {
                return 0;
            }
            calculate_winner_second(player_one, player_two)
        })
        .sum();
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
