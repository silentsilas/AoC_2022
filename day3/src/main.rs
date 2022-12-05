use std::{fs::{File}, env, io::{self, Read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            let part_1_result = solve_part_one(&contents);
            println!("{}", part_1_result);

            let part_2_result = solve_part_two(&contents);
            println!("{}", part_2_result);
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

pub fn solve_part_one(file_contents: &str) -> u32 {
    let rucksacks: Vec<&str> = file_contents.split('\n').collect();
    let mut total_priority: u32 = 0;
    for rucksack in rucksacks {
        let items_length = rucksack.len();
        let (first_compartment, second_compartment) = rucksack.split_at(items_length/2);

        let code = find_common_item_priority([first_compartment, second_compartment].to_vec()).unwrap_or(0);
        total_priority += code;
    }
    total_priority
}

pub fn solve_part_two(file_contents: &str) -> u32 {
    let rucksacks: Vec<&str> = file_contents.split('\n').collect();
    let mut elf_trios = Vec::new();
    for chunk in rucksacks.chunks(3) {
        elf_trios.push(chunk.to_vec());
    }

    let mut total_priority = 0;
    for elf_trio in elf_trios {
        let code: u32 = find_common_item_priority(elf_trio).unwrap_or(0);
        total_priority += code;
    }
    total_priority
}

fn find_common_item_priority(strings: Vec<&str>) -> Option<u32> {
    let mut common_chars = strings[0].chars().collect::<Vec<char>>();

    for s in strings {
        common_chars.retain(|ch| s.contains(*ch));
    }

    match common_chars.len() {
        0 => None,
        _ => Some(convert_ascii_to_priority(common_chars[0]))
    }
}

fn convert_ascii_to_priority(ch: char) -> u32 {
    let code = ch as u32;
    if ch.is_lowercase() {
        code - 96
    } else {
        code - 64 + 26
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
        let result = solve_part_one(&file);
        assert_eq!(result, 157);
    }

    #[test]
    fn solves_part_2() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let result = solve_part_two(&file);
        assert_eq!(result, 70);
    }
}