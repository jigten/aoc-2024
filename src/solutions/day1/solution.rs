use std::{collections::HashMap, fs, i32};

const INPUT_PATH: &str = "src/solutions/day1/input.txt";

pub fn solve_a() {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    let mut res: i32 = 0;

    for (a, b) in left.iter().zip(right.iter()) {
        res += (*b - *a).abs();
    }

    println!("Day 1 A Solution: {res}");
}

pub fn solve_b() {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for n in right {
        counts.entry(n).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut res: i32 = 0;

    for n in left {
        match counts.get(&n) {
            Some(cnt) => res += *cnt * n,
            None => continue,
        }
    }

    println!("Day 1 B Solution: {res}");
}
