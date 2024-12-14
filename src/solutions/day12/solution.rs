use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
    isize,
};

fn get_fence_cost(
    i: usize,
    j: usize,
    garden: &[Vec<char>],
    seen: &mut HashSet<(usize, usize)>,
) -> i32 {
    let current_plant = garden[i][j];
    let mut current_garden = HashSet::from([(i, j)]);
    let mut q = VecDeque::from([(i, j)]);
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();

        for (v, h) in dirs.iter() {
            let dx = x as isize + v;
            let dy = y as isize + h;

            if 0 <= dx
                && dx < garden.len() as isize
                && 0 <= dy
                && dy < garden[0].len() as isize
                && !current_garden.contains(&(dx as usize, dy as usize))
                && garden[dx as usize][dy as usize] == current_plant
            {
                q.push_back((dx as usize, dy as usize));
                current_garden.insert((dx as usize, dy as usize));
            }
        }
    }

    let mut perimeter = 0;

    for (x, y) in current_garden.iter() {
        perimeter += 4;

        for (v, h) in dirs.iter() {
            let dx = *x as isize + v;
            let dy = *y as isize + h;

            if current_garden.contains(&(dx as usize, dy as usize)) {
                perimeter -= 1;
            }
        }
    }

    println!(
        "curent plant {} perimeter {} n {}",
        current_plant,
        perimeter,
        current_garden.len()
    );

    let cost = perimeter * current_garden.len() as i32;
    seen.extend(current_garden);

    cost
}

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for (i, r) in garden.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if !seen.contains(&(i, j)) {
                res += get_fence_cost(i, j, &garden, &mut seen);
            }
        }
    }

    println!("Day 12 A Solution: {res}");
}

fn corners_count(x: usize, y: usize, current_plant: char, garden: &[Vec<char>]) -> usize {
    let mut top_left_corner = true;
    let mut count = 0;
    for (i, j) in [(-1, 0), (0, -1)] {
        let delta_x = x as isize + i;
        let delta_y = y as isize + j;

        if delta_x < 0
            || delta_x >= garden.len() as isize
            || delta_y < 0
            || delta_y >= garden[0].len() as isize
        {
            continue;
        }

        if garden[delta_x as usize][delta_y as usize] == current_plant {
            top_left_corner = false;
            break;
        }
    }

    if top_left_corner {
        count += 1;
    }

    if x + 1 < garden.len()
        && garden[x + 1][y] == current_plant
        && y + 1 < garden[0].len()
        && garden[x][y + 1] == current_plant
        && garden[x + 1][y + 1] != current_plant
    {
        count += 1;
    }

    let mut top_right_corner = true;
    for (i, j) in [(-1, 0), (0, 1)] {
        let delta_x = x as isize + i;
        let delta_y = y as isize + j;

        if delta_x < 0
            || delta_x >= garden.len() as isize
            || delta_y < 0
            || delta_y >= garden[0].len() as isize
        {
            continue;
        }

        if garden[delta_x as usize][delta_y as usize] == current_plant {
            top_right_corner = false;
            break;
        }
    }

    if top_right_corner {
        count += 1;
    }

    if x + 1 < garden.len()
        && garden[x + 1][y] == current_plant
        && y > 0
        && garden[x][y - 1] == current_plant
        && garden[x + 1][y - 1] != current_plant
    {
        count += 1;
    }

    let mut bottom_left_corner = true;
    for (i, j) in [(1, 0), (0, -1)] {
        let delta_x = x as isize + i;
        let delta_y = y as isize + j;

        if delta_x < 0
            || delta_x >= garden.len() as isize
            || delta_y < 0
            || delta_y >= garden[0].len() as isize
        {
            continue;
        }

        if garden[delta_x as usize][delta_y as usize] == current_plant {
            bottom_left_corner = false;
            break;
        }
    }

    if bottom_left_corner {
        count += 1;
    }

    if x > 0
        && garden[x - 1][y] == current_plant
        && y + 1 < garden[0].len()
        && garden[x][y + 1] == current_plant
        && garden[x - 1][y + 1] != current_plant
    {
        count += 1;
    }

    let mut bottom_right_corner = true;
    for (i, j) in [(1, 0), (0, 1)] {
        let delta_x = x as isize + i;
        let delta_y = y as isize + j;

        if delta_x < 0
            || delta_x >= garden.len() as isize
            || delta_y < 0
            || delta_y >= garden[0].len() as isize
        {
            continue;
        }

        if garden[delta_x as usize][delta_y as usize] == current_plant {
            bottom_right_corner = false;
            break;
        }
    }

    if bottom_right_corner {
        count += 1;
    }

    if x as isize > 0
        && garden[x - 1][y] == current_plant
        && y as isize > 0
        && garden[x][y - 1] == current_plant
        && garden[x - 1][y - 1] != current_plant
    {
        count += 1;
    }
    println!("{current_plant} at {x},{y} has count: {}", count);

    count
}

fn get_fence_cost_discount(
    i: usize,
    j: usize,
    garden: &[Vec<char>],
    seen: &mut HashSet<(usize, usize)>,
) -> i32 {
    let current_plant = garden[i][j];
    let mut distinct_sides = 0;
    let mut current_garden = HashSet::from([(i, j)]);
    let mut q = VecDeque::from([(i, j)]);
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        distinct_sides += corners_count(x, y, current_plant, garden);

        for (v, h) in dirs.iter() {
            let dx = x as isize + v;
            let dy = y as isize + h;

            if 0 <= dx
                && dx < garden.len() as isize
                && 0 <= dy
                && dy < garden[0].len() as isize
                && !current_garden.contains(&(dx as usize, dy as usize))
                && garden[dx as usize][dy as usize] == current_plant
            {
                q.push_back((dx as usize, dy as usize));
                current_garden.insert((dx as usize, dy as usize));
            }
        }
    }

    let cost = current_garden.len() * distinct_sides;
    println!("{current_plant} has {distinct_sides} sides");
    seen.extend(current_garden);

    cost.try_into().unwrap()
}

pub fn solve_b() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for (i, r) in garden.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if !seen.contains(&(i, j)) {
                res += get_fence_cost_discount(i, j, &garden, &mut seen);
            }
        }
    }

    println!("Day 12 B Solution {res}");
}
