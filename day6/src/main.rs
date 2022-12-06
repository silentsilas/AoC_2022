use std::{env, fs};

fn main() {
    let file_path: &String = &env::args().collect::<Vec<String>>()[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let part_one_results = solve(&contents, 4);
    println!("Start packet marker: {:?}", part_one_results.unwrap());

    let part_two_results = solve(&contents, 14);
    println!("Start message marker: {:?}", part_two_results.unwrap());
}

pub fn solve(data_stream: &str, start_marker_size: usize) -> Option<usize> {
    let input = data_stream.chars().collect::<Vec<char>>();
    let rolling_history = input.windows(start_marker_size);

    for (idx, current) in rolling_history.enumerate() {
        let found_duplicate = current.iter().any(|ch| {
                current.iter()
                    .filter(|other_char| *ch == **other_char)
                    .count() > 1
        });

        if !found_duplicate {
            return Some(idx + start_marker_size)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn solves_part_1() {
        const EXPECTED_RESULTS: [usize; 5] = [7, 5, 6, 10, 11];
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        for (idx, _) in EXPECTED_RESULTS.iter().enumerate() {
            let path = dir.to_owned() + "/assets/part_one/test" + &(idx + 1).to_string() +".txt";            
            let file = fs::read_to_string(path).unwrap();
            let result = solve(&file, 4);
            assert_eq!(result, Some(EXPECTED_RESULTS[idx]));
        }
    }

    #[test]
    fn solves_part_2() {
        const EXPECTED_RESULTS: [usize; 5] = [19, 23, 23, 29, 26];
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        for (idx, _) in EXPECTED_RESULTS.iter().enumerate() {
            let path = dir.to_owned() + "/assets/part_two/test" + &(idx + 1).to_string() +".txt";            
            let file = fs::read_to_string(path).unwrap();
            let result = solve(&file, 14);
            assert_eq!(result, Some(EXPECTED_RESULTS[idx]));
        }
    }
}