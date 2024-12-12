use core::prelude;
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn transform_stones(stones: &[String]) -> Vec<String> {
    let mut transformed = Vec::new();

    for stone in stones.iter() {
        if stone == "0" {
            transformed.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let mid = stone.len() / 2;
            let first_half = &stone[0..mid];
            let second_half = &stone[mid..];

            let mut first_half = first_half.trim_start_matches('0').to_string();
            let mut second_half = second_half.trim_start_matches('0').to_string();

            if first_half.is_empty() {
                first_half = "0".to_string();
            }

            if second_half.is_empty() {
                second_half = "0".to_string();
            }
            transformed.push(first_half);
            transformed.push(second_half);
        } else {
            let new_value = (stone.parse::<i64>().unwrap() * 2024).to_string();
            transformed.push(new_value);
        }
    }

    transformed
}

pub fn solve_a() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut stones: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();

    for i in 0..25 {
        stones = transform_stones(&stones);
    }

    println!("Day 11 A Solution: {}", stones.len());
}

fn transform_stones_b() {}

pub fn solve_b() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut stones: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let mut memo: HashMap<String, u64> = HashMap::new();

    for stone in stones.iter() {
        let counts = memo.entry(stone.to_string()).or_insert(0);
        *counts += 1;
    }

    for i in 0..75 {
        let mut temp: HashMap<String, u64> = HashMap::new();
        for (stone, val) in &memo {
            if stone == "0" {
                let counts = temp.entry("1".to_string()).or_insert(0);
                *counts += val;
            } else if stone.len() % 2 == 0 {
                let mid = stone.len() / 2;
                let first_half = &stone[0..mid];
                let second_half = &stone[mid..];

                let mut first_half = first_half.trim_start_matches('0').to_string();
                let mut second_half = second_half.trim_start_matches('0').to_string();

                if first_half.is_empty() {
                    first_half = "0".to_string();
                }

                if second_half.is_empty() {
                    second_half = "0".to_string();
                }

                let count_first = temp.entry(first_half.to_string()).or_insert(0);
                *count_first += val;

                let count_second = temp.entry(second_half.to_string()).or_insert(0);
                *count_second += val;
            } else {
                let new_value = (stone.parse::<i64>().unwrap() * 2024).to_string();
                let count = temp.entry(new_value.to_string()).or_insert(0);
                *count += val;
            }
        }
        memo = temp;
    }
    let mut res = 0;

    for v in memo.values() {
        res += v;
    }

    println!("Day 11 A Solution: {res}");
}
