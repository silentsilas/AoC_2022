use std::{fs::{File}, env, io::{self, Read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            let elves = get_elf_calories(contents);
            let part_two_elves = elves.clone();
            let part_one_results = solve_part_1(elves);
            let part_two_results = solve_part_2(part_two_elves);

            println!("Elf with most calories: {}", part_one_results);
            println!("Sum of three elves with most calories: {}", part_two_results);
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

pub fn get_elf_calories(file_contents: String) -> Vec<i32> {
    let elf_lines: Vec<&str> = file_contents.split("\n\n").collect();
    let mut elves = Vec::new();
    for elf in elf_lines {
        let mut total_calories = 0;
        for calories in elf.split('\n') {
            total_calories += calories.parse::<i32>().unwrap_or(0);
        }
        elves.push(total_calories)
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves
}

pub fn solve_part_1(elves: Vec<i32>) -> i32 {
    elves[0]
}

pub fn solve_part_2(elves: Vec<i32>) -> i32 {
    let result: i32 = elves[0..3].iter().sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let elves = get_elf_calories(file);
        let result = solve_part_1(elves);
        assert_eq!(result, 120);
    }

    #[test]
    fn solves_part_2() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = read_file(&path).unwrap();
        let elves = get_elf_calories(file);
        let result = solve_part_2(elves);
        assert_eq!(result, 250);
    }
}
