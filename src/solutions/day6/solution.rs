use std::collections::HashSet;
use std::io::{self, Read};

pub fn solve_a() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut guard_direction: usize = 0;

    let mut guard_position: (i32, i32) = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|x| *x == '^') {
            guard_position = (r as i32, c as i32);
            break;
        }
    }

    let mut visited = HashSet::from([guard_position]);

    // Simulate the guard's movement
    loop {
        let next_position = (
            guard_position.0 + dirs[guard_direction].0,
            guard_position.1 + dirs[guard_direction].1,
        );

        // Check bounds
        if next_position.0 < 0
            || next_position.0 >= grid.len() as i32
            || next_position.1 < 0
            || next_position.1 >= grid[0].len() as i32
        {
            break;
        }

        // If the guard encounters a wall, rotate direction
        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            guard_direction = (guard_direction + 1) % dirs.len();
        } else {
            // Otherwise, move to the next position
            guard_position = next_position;
            if visited.contains(&guard_position) {
                continue;
            }
            visited.insert(guard_position);
        }
    }
    println!("Day 6 Solution A: {}", visited.len());
}

fn simulate(grid: &[Vec<char>], guard_start_position: &(i32, i32)) -> bool {
    // Simulate the guard's movement
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut guard_direction: usize = 0;
    let mut guard_position = *guard_start_position;
    let mut seen: HashSet<(i32, i32, usize)> =
        HashSet::from([(guard_position.0, guard_position.1, guard_direction)]);

    while guard_position.0 >= 0
        && guard_position.0 < grid.len() as i32
        && guard_position.1 >= 0
        && guard_position.1 < grid[0].len() as i32
    {
        let next_position = (
            guard_position.0 + dirs[guard_direction].0,
            guard_position.1 + dirs[guard_direction].1,
        );

        // Check bounds
        if next_position.0 < 0
            || next_position.0 >= grid.len() as i32
            || next_position.1 < 0
            || next_position.1 >= grid[0].len() as i32
        {
            return false;
        }

        // If the guard encounters a wall, rotate direction
        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            guard_direction = (guard_direction + 1) % dirs.len();
        } else {
            // Otherwise, move to the next position
            guard_position = next_position;

            if seen.contains(&(guard_position.0, guard_position.1, guard_direction)) {
                return true;
            }
            seen.insert((guard_position.0, guard_position.1, guard_direction));
        }
    }

    false
}

pub fn solve_b() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut guard_start_position: (i32, i32) = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|x| *x == '^') {
            guard_start_position = (r as i32, c as i32);
            break;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                grid[i][j] = '#';

                if simulate(&grid, &guard_start_position) {
                    res += 1;
                }

                grid[i][j] = '.'
            }
        }
    }

    println!("Day 6 Solution B: {res}");
}
