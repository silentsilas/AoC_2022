use std::{fs::{File}, env, io::{self, Read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            let part_1_result = solve(contents);
            println!("{}", part_1_result);
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

pub fn solve(file_contents: String) -> u32 {
    let rucksacks: Vec<&str> = file_contents.split('\n').collect();
    let mut total_priority: u32 = 0;
    for rucksack in rucksacks {
        let items_length = rucksack.len();
        let (first_compartment, second_compartment) = rucksack.split_at(items_length/2);

        let code = find_common_item_priority(first_compartment, second_compartment).unwrap_or(0);
        total_priority += code;
    }
    total_priority
}

fn find_common_item_priority(s1: &str, s2: &str) -> Option<u32> {
    for ch in s1.chars() {
        let code = ch as u32;
        if s2.contains(ch) {
            if ch.is_lowercase() {
                return Some(code - 96);
            } else {
                return Some(code - 64 + 26);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let result = solve(file);
        assert_eq!(result, 157);
    }
}