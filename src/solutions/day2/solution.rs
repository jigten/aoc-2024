use std::{fs, i32};

const INPUT_PATH: &str = "src/solutions/day2/input.txt";

pub fn solve_a() {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut res = 0;

    for line in contents.lines() {
        let level: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        let mut is_valid = true;
        for i in 1..level.len() {
            let distance = (level[i] - level[i - 1]).abs();

            if !(1..=3).contains(&distance) {
                is_valid = false;
                break;
            }
        }

        if is_valid
            && (level.windows(2).all(|w| w[0] < w[1]) || level.windows(2).all(|w| w[0] > w[1]))
        {
            res += 1;
        }
    }

    println!("Day 2 A Solution: {res}");
}

fn is_safe(report: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if diff < 0 {
            is_increasing = false;
        }

        if diff > 0 {
            is_decreasing = false;
        }

        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }

    is_increasing | is_decreasing
}

//fn is_safe(level: &[i32]) -> bool {
//    if level.len() < 2 {
//        return true;
//    }
//
//    let mut increasing = true;
//    let mut decreasing = true;
//
//    for i in 1..level.len() {
//        let diff = level[i] - level[i - 1];
//
//        if !(1..=3).contains(&diff.abs()) {
//            return false; // Adjacent levels differ by more than 1 to 3
//        }
//
//        if diff > 0 {
//            decreasing = false; // Not decreasing
//        }
//        if diff < 0 {
//            increasing = false; // Not increasing
//        }
//    }
//
//    increasing | decreasing
//}

pub fn solve_b() {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut res = 0;

    fn test(report: &[i32], i: usize) -> bool {
        let mut new_report = report.to_vec();
        new_report.remove(i);

        is_safe(&new_report)
    }

    for line in contents.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        let mut any_ok = false;

        if test(&report, 0) {
            any_ok = true;
        }

        for i in 0..report.len() - 1 {
            let diff = report[i + 1] - report[i];

            if diff.abs() < 1 || diff.abs() > 3 {
                if test(&report, i) {
                    any_ok = true;
                }

                if test(&report, i + 1) {
                    any_ok = true;
                }
                break;
            }

            if i + 2 < report.len() {
                let diff_two = report[i + 2] - report[i + 1];
                if (diff > 0) != (diff_two > 0) {
                    if test(&report, i) {
                        any_ok = true;
                    }
                    if test(&report, i + 1) {
                        any_ok = true;
                    }
                    if test(&report, i + 2) {
                        any_ok = true;
                    }
                    break;
                }
            }
        }

        if any_ok {
            res += 1;
        }
    }

    println!("Day 2 B Solution: {res}");
}
