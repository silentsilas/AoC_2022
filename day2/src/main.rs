use std::{fs::{File}, env, io::{self, Read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            let part_1_result = solve(&contents, "Move");
            println!("Part 1 total score: {}", part_1_result);

            let part_2_result = solve(&contents, "Outcome");
            println!("Part 2 total score: {}", part_2_result);
        },
        Err(e) => eprintln!("{}", e)
    }
}

pub fn read_file(file_path: &String) -> Result<String, io::Error> {
    let file_result = File::open(file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e)
    }
}

pub fn solve(file_contents: &str, strategy: &str) -> i32 {
    let rounds: Vec<&str> = file_contents.split('\n').collect();
    let mut total_score = 0;
    for round in rounds {
        let moves: Vec<&str> = round.split(' ').collect();
        let opponent_move = match moves[0] {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            _ => continue
        };
        let player_move = decipher_player_code(opponent_move, moves[1], strategy);

        let score = calculate_round_score(opponent_move, player_move);
        total_score += score;
    }
    total_score
}

fn decipher_player_code<'a>(opponent_move: &'a str, player_code: &'a str, strategy: &'a str) -> (&'a str, i32) {
    match strategy {
        "Move" => decipher_player_move(player_code),
        "Outcome" => decipher_player_outcome(opponent_move, player_code),
        _ => ("", 0)
    }
}

fn decipher_player_move(player_code: &str) -> (&str, i32) {
    match player_code {
        "X" => ("Rock", 1),
        "Y" => ("Paper", 2),
        "Z" => ("Scissors", 3),
        _ => ("", 0)
    }
}

fn decipher_player_outcome<'a>(opponent_move: &'a str, player_code: &'a str) -> (&'a str, i32) {
    let result: (&str, i32) = match opponent_move {
        "Rock" => {
            match player_code {
                "X" => ("Scissors", 3),
                "Y" => ("Rock", 1),
                "Z" => ("Paper", 2),
                _ => ("", 0)
            }
        },
        "Paper" => {
            match player_code {
                "X" => ("Rock", 1),
                "Y" => ("Paper", 2),
                "Z" => ("Scissors", 3),
                _ => ("", 0)
            }
        },
        "Scissors" => {
            match player_code {
                "X" => ("Paper", 2),
                "Y" => ("Scissors", 3),
                "Z" => ("Rock", 1),
                _ => ("", 0)
            }
        },
        _ => ("", 0)
    };

    result
}

pub fn calculate_round_score(opponent_move: &str, player_move: (&str, i32)) -> i32 {
    match opponent_move {
        "Rock" => {
            match player_move.0 {
                "Rock" => 3 + player_move.1,
                "Paper" => 6 + player_move.1,
                "Scissors" => player_move.1,
                _ => 0
            }
        },
        "Paper" => {
            match player_move.0 {
                "Rock" => player_move.1,
                "Paper" => 3 + player_move.1,
                "Scissors" => 6 + player_move.1,
                _ => 0
            }
        },
        "Scissors" => {
            match player_move.0 {
                "Rock" => 6 + player_move.1,
                "Paper" => player_move.1,
                "Scissors" => 3 + player_move.1,
                _ => 0
            }
        },
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let result = solve(&file, "Move");
        assert_eq!(result, 74);
    }

    #[test]
    fn solves_part_2() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let result = solve(&file, "Outcome");
        assert_eq!(result, 57);
    }
}
