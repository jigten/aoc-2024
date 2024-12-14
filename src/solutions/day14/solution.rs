use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_robot_details = Regex::new(r"p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if let Some(cap) = re_robot_details.captures(line) {
            let robot: Vec<i32> = (1..=4).map(|i| cap[i].parse::<i32>().unwrap()).collect();
            robots.push(robot);
        }
    }

    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;

    let height = 102;
    let width = 100;

    let center_x = 50;
    let center_y = 51;

    for robot in robots.iter() {
        let x = robot[0];
        let y = robot[1];
        let v_x = robot[2];
        let v_y = robot[3];

        let new_x = ((x + (v_x * 100)) % (width + 1) + (width + 1)) % (width + 1);
        let new_y = ((y + (v_y * 100)) % (height + 1) + (height + 1)) % (height + 1);

        if new_x > center_x && new_y > center_y {
            q1_count += 1; // Top-right
        } else if new_x < center_x && new_y > center_y {
            q2_count += 1; // Top-left
        } else if new_x < center_x && new_y < center_y {
            q3_count += 1; // Bottom-left
        } else if new_x > center_x && new_y < center_y {
            q4_count += 1; // Bottom-right
        }
    }

    println!(
        "Day 14 A Solution: {}",
        q1_count * q2_count * q3_count * q4_count
    );
}

pub fn solve_b() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_robot_details = Regex::new(r"p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if let Some(cap) = re_robot_details.captures(line) {
            let robot: Vec<i32> = (1..=4).map(|i| cap[i].parse::<i32>().unwrap()).collect();
            robots.push(robot);
        }
    }

    let height = 102;
    let width = 100;

    let center_x = 50;
    let center_y = 51;

    for i in 1..=10000 {
        let mut seen = HashSet::new();
        let mut all_unique = true;

        for robot in robots.iter() {
            let x = robot[0];
            let y = robot[1];
            let v_x = robot[2];
            let v_y = robot[3];

            let new_x = ((x + (v_x * i)) % (width + 1) + (width + 1)) % (width + 1);
            let new_y = ((y + (v_y * i)) % (height + 1) + (height + 1)) % (height + 1);

            if seen.contains(&(new_x, new_y)) {
                all_unique = false;
                continue;
            }

            seen.insert((new_x, new_y));
        }

        if all_unique {
            println!("Day 14 B Solution: {i}");
        }
    }
}
