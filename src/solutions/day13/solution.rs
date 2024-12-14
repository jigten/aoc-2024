use regex::Regex;
use std::cmp::min;
use std::io::{self, Read};

fn calc_min_cost(a: (usize, usize), b: (usize, usize), prize: (usize, usize)) -> i64 {
    let (target_x, target_y) = prize;
    let mut dp = vec![vec![i64::MAX; target_y + 1]; target_x + 1];
    dp[0][0] = 0;

    for x in 0..=target_x {
        for y in 0..=target_y {
            if dp[x][y] == i64::MAX {
                continue;
            }
            // Apply operation a
            if x + a.0 <= target_x && y + a.1 <= target_y {
                dp[x + a.0][y + a.1] = min(dp[x + a.0][y + a.1], dp[x][y] + 3);
            }
            // Apply operation b
            if x + b.0 <= target_x && y + b.1 <= target_y {
                dp[x + b.0][y + b.1] = min(dp[x + b.0][y + b.1], dp[x][y] + 1);
            }
        }
    }

    if dp[target_x][target_y] == i64::MAX {
        return 0;
    }

    dp[target_x][target_y]
}

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_button = Regex::new(r"X\+(\d+),\s*Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"X=(\d+),\s*Y=(\d+)").unwrap();

    let mut results = Vec::new();
    let mut a = None;
    let mut b = None;
    let mut prize = None;

    for line in input.lines() {
        if let Some(cap) = re_button.captures(line) {
            let x = cap[1].parse::<usize>().unwrap();
            let y = cap[2].parse::<usize>().unwrap();
            if a.is_none() {
                a = Some((x, y));
            } else {
                b = Some((x, y));
            }
        } else if let Some(cap) = re_prize.captures(line) {
            let x = cap[1].parse::<usize>().unwrap();
            let y = cap[2].parse::<usize>().unwrap();
            prize = Some((x, y));
        }

        if a.is_some() && b.is_some() && prize.is_some() {
            results.push((a.unwrap(), b.unwrap(), prize.unwrap()));
            a = None;
            b = None;
            prize = None;
        }
    }

    for (a, b, prize) in results.iter() {
        res += calc_min_cost(*a, *b, *prize);
    }

    println!("Day 13 A Solution: {res}");
}

fn solve_system_of_equations(
    a1: f64,
    b1: f64,
    c1: f64,
    a2: f64,
    b2: f64,
    c2: f64,
) -> Option<(f64, f64)> {
    // Calculate the determinant of the coefficient matrix A
    let det_a = a1 * b2 - a2 * b1;

    // If the determinant is zero, the system does not have a unique solution
    if det_a == 0.0 {
        return None; // No unique solution
    }

    // Calculate the determinant of the matrices A_x and A_y
    let det_ax = c1 * b2 - c2 * b1;
    let det_ay = a1 * c2 - a2 * c1;

    // Calculate the solutions for x and y using Cramer's rule
    let x = det_ax / det_a;
    let y = det_ay / det_a;

    Some((x, y)) // Return the solution as a tuple (x, y)
}

pub fn solve_b() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_button = Regex::new(r"X\+(\d+),\s*Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"X=(\d+),\s*Y=(\d+)").unwrap();

    let mut results = Vec::new();
    let mut a = None;
    let mut b = None;
    let mut prize = None;

    for line in input.lines() {
        if let Some(cap) = re_button.captures(line) {
            let x = cap[1].parse::<i64>().unwrap();
            let y = cap[2].parse::<i64>().unwrap();
            if a.is_none() {
                a = Some((x, y));
            } else {
                b = Some((x, y));
            }
        } else if let Some(cap) = re_prize.captures(line) {
            let x = cap[1].parse::<i64>().unwrap();
            let y = cap[2].parse::<i64>().unwrap();
            prize = Some((x + 10000000000000, y + 10000000000000));
        }

        if a.is_some() && b.is_some() && prize.is_some() {
            results.push((a.unwrap(), b.unwrap(), prize.unwrap()));
            a = None;
            b = None;
            prize = None;
        }
    }

    for (a, b, prize) in results.iter() {
        if let Some(potential_res) = solve_system_of_equations(
            a.0 as f64,
            b.0 as f64,
            prize.0 as f64,
            a.1 as f64,
            b.1 as f64,
            prize.1 as f64,
        ) {
            let a_pushes = potential_res.0 as i64;
            let b_pushes = potential_res.1 as i64;

            if a.0 * a_pushes + b.0 * b_pushes == prize.0 {
                res += (a_pushes * 3 + b_pushes);
            }
        }
    }

    println!("Day 13 B Solution: {res}");
}
