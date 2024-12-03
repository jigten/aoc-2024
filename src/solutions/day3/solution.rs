use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "src/solutions/day3/input.txt";

fn get_valid_product_sum(line: &str) -> i32 {
    let mut res = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for caps in re.captures_iter(line) {
        if let (Ok(x), Ok(y)) = (caps[1].parse::<i32>(), caps[2].parse::<i32>()) {
            res += x * y;
        }
    }

    res
}

pub fn solve_a() {
    let mut res = 0;
    let memory = fs::read_to_string(INPUT_PATH).unwrap();

    for line in memory.lines() {
        res += get_valid_product_sum(line);
    }

    println!("Day 3 A Solution: {res}");
}

pub fn solve_b() {
    let mut res = 0;
    let memory = fs::read_to_string(INPUT_PATH).unwrap();
    let mut enabled = true;

    for (i, c) in memory.chars().enumerate() {
        if i + 4 < memory.len() && &memory[i..i + 4] == "do()" {
            enabled = true;
        }

        if i + 7 < memory.len() && &memory[i..i + 7] == "don't()" {
            enabled = false;
        }

        if c == 'm' && enabled {
            for j in (6..=11).rev() {
                if i + j < memory.len() - 1 {
                    res += get_valid_product_sum(&memory[i..=i + j]);
                    break;
                }
            }
        }
    }

    println!("Day 3 B Solution: {res}");
}
