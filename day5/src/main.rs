use std::{env, fs, collections::VecDeque};
use itertools::Itertools;
use scan_fmt::scan_fmt;

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize
}

pub enum Strategy {
    Part1,
    Part2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();
    let first_result = solve(&contents, Strategy::Part1);
    let second_result = solve(&contents, Strategy::Part2);
    println!("Part 1 Solution: {}", first_result);
    println!("Part 2 Solution: {}", second_result);
}

pub fn solve(contents: &str, strategy: Strategy) -> String {
    let (mut stacks, instructions) = parse_crates_and_instructions(contents);

    for instruction in instructions {
        match strategy {
            Strategy::Part1 => {
                for _ in 0..instruction.quantity {
                    let item = stacks[instruction.from].pop_back().unwrap();
                    stacks[instruction.to].push_back(item)
                }
            },
            Strategy::Part2 => {
                let mut crane_stack: VecDeque<char> = VecDeque::new();
                for _ in 0..instruction.quantity {
                    let item = stacks[instruction.from].pop_back().unwrap();
                    crane_stack.push_back(item)
                }

                for _ in 0..instruction.quantity {
                    let item = crane_stack.pop_back().unwrap();
                    stacks[instruction.to].push_back(item)
                }
            }
        };
    }

    let mut result = "".to_owned();
    for mut stack in stacks {
        result.push_str(&stack.pop_back().unwrap().to_string());
    };

    result
}

fn parse_crates_and_instructions(contents: &str) -> (Vec<VecDeque<char>>, Vec<Instruction>) {
    let (crates, instructions) = contents.split("\n\n").into_iter().collect_tuple::<(&str, &str)>().unwrap();
    let crate_stacks = parse_crates(crates);
    let instructions = parse_instructions(instructions);

    (crate_stacks, instructions)
}

fn parse_crates(crates: &str) -> Vec<VecDeque<char>> {
    let rows: Vec<&str> = crates.lines().collect::<Vec<&str>>();
    let width: usize = get_width(rows.clone().first().unwrap());
    let mut crate_stacks: Vec<VecDeque<char>> = vec![];

    for _ in 0..width {
        let stack = VecDeque::new();
        crate_stacks.push(stack);
    }

    for row in rows {
        if row.contains(" 1") {
            continue;
        }

        for (col, ch) in row.chars().skip(1).step_by(4).enumerate() {
            let stack = &mut crate_stacks[col];

            if ch != ' ' {
                stack.push_front(ch);
            }
        }
    }

    crate_stacks
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = instructions
            .lines()
            .filter_map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).ok())
            .map(|(quantity, from, to)| Instruction { 
                quantity, from: from - 1, to: to - 1 
            })
            .collect();

            instructions
}
fn get_width(row: &str) -> usize {
    row.chars().skip(1).step_by(4).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let dir = env!("CARGO_MANIFEST_DIR").to_string();
        let path = dir + "/assets/test.txt";
        let file = fs::read_to_string(path).unwrap();
        let first_result = solve(&file, Strategy::Part1);
        assert_eq!(first_result, "CMZ");

        let second_result = solve(&file, Strategy::Part2);
        assert_eq!(second_result, "MCD");
    }
}