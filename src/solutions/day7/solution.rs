use std::io::{self, Read};

fn parse_calibration_equation(line: &str) -> (i64, Vec<i64>) {
    let parts = line.split_once(':').unwrap();
    let test_value: i64 = parts.0.parse().unwrap();
    let equation: Vec<i64> = parts.1.split(' ').filter_map(|x| x.parse().ok()).collect();

    (test_value, equation)
}

fn is_valid(current_value: i64, i: usize, test_value: i64, equation: &[i64]) -> bool {
    if current_value > test_value {
        return false;
    }

    if i == equation.len() {
        return current_value == test_value;
    }

    is_valid(current_value + equation[i], i + 1, test_value, equation)
        || is_valid(current_value * equation[i], i + 1, test_value, equation)
}

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let (test_value, equation) = parse_calibration_equation(line);

        if is_valid(equation[0], 1, test_value, &equation) {
            res += test_value;
        }
    }

    println!("Day 7 A Solution: {res}");
}

fn concat_i64(a: i64, b: i64) -> i64 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<i64>().unwrap()
}

fn is_valid_with_concat(current_value: i64, i: usize, test_value: i64, equation: &[i64]) -> bool {
    if current_value > test_value {
        return false;
    }

    if i >= equation.len() {
        return current_value == test_value;
    }

    is_valid_with_concat(
        concat_i64(current_value, equation[i]),
        i + 1,
        test_value,
        equation,
    ) || is_valid_with_concat(current_value + equation[i], i + 1, test_value, equation)
        || is_valid_with_concat(current_value * equation[i], i + 1, test_value, equation)
}

pub fn solve_b() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let (test_value, equation) = parse_calibration_equation(line);

        if is_valid_with_concat(equation[0], 1, test_value, &equation) {
            res += test_value;
        }
    }

    println!("Day 7 B Solution: {res}");
}
