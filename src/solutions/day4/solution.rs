use std::fs;

const INPUT_PATH: &str = "src/solutions/day4/input.txt";

fn count_xmas(i: usize, j: usize, grid: &Vec<Vec<char>>) -> i32 {
    let dirs: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for &(dx, dy) in &dirs {
        let mut valid = true;

        for k in 0..target.len() {
            let x = i as i32 + k as i32 * dx;
            let y = j as i32 + k as i32 * dy;

            if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
                valid = false;
                break;
            }

            let x = x as usize;
            let y = y as usize;

            if grid[x][y] != target[k] {
                valid = false;
                break;
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

pub fn solve_a() {
    let mut res = 0;
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                res += count_xmas(i, j, &grid);
            }
        }
    }

    println!("Day 4 A Solution: {res}");
}

fn is_mas(i: usize, j: usize, grid: &[Vec<char>]) -> bool {
    let top_right_and_bottom_left = format!("{}{}{}", grid[i + 1][j - 1], 'A', grid[i - 1][j + 1]);
    let top_left_and_bottom_right = format!("{}{}{}", grid[i - 1][j - 1], 'A', grid[i + 1][j + 1]);

    let contents = ["SAM", "MAS"];

    // Check if both strings are present in `contents`
    contents.contains(&top_left_and_bottom_right.as_str())
        && contents.contains(&top_right_and_bottom_left.as_str())
}

pub fn solve_b() {
    let mut res = 0;

    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 'A' && is_mas(i, j, &grid) {
                println!("{i},{j}");
                res += 1;
            }
        }
    }

    println!("Day 4 B Solution: {res}");
}
