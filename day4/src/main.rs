use std::{fs, env};


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    
    let (part_1_result, part_2_result) = solve(&contents);
    println!("{}", part_1_result);
    println!("{}", part_2_result);
}

pub fn solve(contents: &str) -> (i32, i32) {
    let pairs: Vec<&str> = contents.split('\n').collect();
    let mut part_1_result = 0;
    let mut part_2_result: i32 = 0;
    for pair in pairs {
        if let [elf1, elf2] = pair.split(',').collect::<Vec<&str>>()[..] {

            let elf_range1 = parse_elf_rooms(elf1);
            let elf_range2 = parse_elf_rooms(elf2);
            if contains_range(elf_range1, elf_range2) || contains_range(elf_range2, elf_range1) {
                part_1_result += 1;
            }

            if intersects_range(elf_range1, elf_range2){
                part_2_result += 1;
            }

        } else {
            continue;
        }
    }
    (part_1_result, part_2_result)
}

fn parse_elf_rooms(elf: &str) -> (i32, i32) {
    if let [start, end] = elf.split('-').collect::<Vec<&str>>()[..] {
        (start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap())
    } else {
        (0, 0)
    }
}

// checks if first range fully contains second range
fn contains_range(range1: (i32, i32), range2: (i32, i32)) -> bool {
    let (low1, high1) = range1;
    let (low2, high2) = range2;

    low1 <= low2 && high1 >= high2
}

fn intersects_range(range1: (i32, i32), range2: (i32, i32)) -> bool {
    let (low1, high1) = range1;
    let (low2, high2) = range2;

    low1 <= high2 && high1 >= low2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = fs::read_to_string(path).unwrap();
        let (first_result, second_result) = solve(&file);
        assert_eq!(first_result, 2);
        assert_eq!(second_result, 4);
    }
}