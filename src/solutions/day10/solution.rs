use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

fn get_trailhead_score(r: usize, c: usize, grid: &[Vec<i32>]) -> i32 {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q = VecDeque::from([(r, c)]);
    let mut seen = HashSet::from([(r, c)]);
    let mut count = 0;

    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        let curr_height = grid[i][j];

        for (x, y) in dirs {
            let new_i = i as isize + x;
            let new_j = j as isize + y;

            if 0 <= new_i
                && new_i < grid.len() as isize
                && 0 <= new_j
                && new_j < grid[new_i as usize].len() as isize
                && !seen.contains(&(new_i as usize, new_j as usize))
                && grid[new_i as usize][new_j as usize] == curr_height + 1
            {
                if grid[new_i as usize][new_j as usize] == 9 {
                    count += 1;
                } else {
                    q.push_back((new_i as usize, new_j as usize));
                }
                // uncomment below for solution to part a
                // seen.insert((new_i as usize, new_j as usize));
            }
        }
    }

    count
}

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().map(|x| (x as i32) - ('0' as i32)).collect());
    }

    for (r, row) in grid.iter().enumerate() {
        for (c, height) in grid[r].iter().enumerate() {
            if *height == 0 {
                res += get_trailhead_score(r, c, &grid);
            }
        }
    }

    println!("Day 10 A Solution: {res}");
}
