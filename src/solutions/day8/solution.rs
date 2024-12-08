use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn add_antinodes(
    position: &[(usize, usize)],
    anti_nodes: &mut HashSet<(usize, usize)>,
    r_limit: usize,
    c_limit: usize,
) {
    for i in 0..position.len() {
        for j in i + 1..position.len() {
            let x = position[i];
            let y = position[j];

            let direction_vec: (isize, isize) =
                ((y.0 as isize - x.0 as isize), (y.1 as isize - x.1 as isize));
            let anti_node_one = (
                y.0 as isize + direction_vec.0,
                y.1 as isize + direction_vec.1,
            );
            let anti_node_two = (
                x.0 as isize - direction_vec.0,
                x.1 as isize - direction_vec.1,
            );

            if !anti_nodes.contains(&(anti_node_one.0 as usize, anti_node_one.1 as usize))
                && anti_node_one.0 >= 0
                && anti_node_one.0 < r_limit as isize
                && anti_node_one.1 >= 0
                && anti_node_one.1 < c_limit as isize
            {
                anti_nodes.insert((anti_node_one.0 as usize, anti_node_one.1 as usize));
            }

            if !anti_nodes.contains(&(anti_node_two.0 as usize, anti_node_two.1 as usize))
                && anti_node_two.0 >= 0
                && anti_node_two.0 < r_limit as isize
                && anti_node_two.1 >= 0
                && anti_node_two.1 < c_limit as isize
            {
                anti_nodes.insert((anti_node_two.0 as usize, anti_node_two.1 as usize));
            }
        }
    }
}

pub fn solve_a() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut graph: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        graph.push(line.chars().collect());
    }

    let mut freq_position_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (r, line) in graph.iter().enumerate() {
        for (c, character) in line.iter().enumerate() {
            if character.is_ascii_digit() {
                freq_position_map
                    .entry(*character)
                    .or_default()
                    .push((r, c));
            }

            if character.is_ascii_alphabetic() {
                freq_position_map
                    .entry(*character)
                    .or_default()
                    .push((r, c));
            }
        }
    }

    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, position) in freq_position_map.iter() {
        add_antinodes(position, &mut anti_nodes, graph.len(), graph[0].len());
    }

    println!("Day 8 A Solution: {}", anti_nodes.len());
}

fn add_antinodes_resonant(
    position: &[(usize, usize)],
    anti_nodes: &mut HashSet<(usize, usize)>,
    r_limit: usize,
    c_limit: usize,
) {
    for i in 0..position.len() {
        for j in i + 1..position.len() {
            let x = position[i];
            let y = position[j];
            anti_nodes.insert((x.0, x.1));
            anti_nodes.insert((y.0, y.1));

            let direction_vec: (isize, isize) =
                ((y.0 as isize - x.0 as isize), (y.1 as isize - x.1 as isize));

            let mut delta = 1;
            loop {
                let mut next_antinode = (
                    y.0 as isize + (delta * direction_vec.0),
                    y.1 as isize + (delta * direction_vec.1),
                );

                if !(next_antinode.0 >= 0
                    && next_antinode.0 < r_limit as isize
                    && next_antinode.1 >= 0
                    && next_antinode.1 < c_limit as isize)
                {
                    break;
                }

                if !anti_nodes.contains(&(next_antinode.0 as usize, next_antinode.1 as usize)) {
                    anti_nodes.insert((next_antinode.0 as usize, next_antinode.1 as usize));
                }

                delta += 1;
            }

            delta = 1;
            loop {
                let mut next_antinode = (
                    x.0 as isize - (delta * direction_vec.0),
                    x.1 as isize - (delta * direction_vec.1),
                );

                if !(next_antinode.0 >= 0
                    && next_antinode.0 < r_limit as isize
                    && next_antinode.1 >= 0
                    && next_antinode.1 < c_limit as isize)
                {
                    break;
                }

                if !anti_nodes.contains(&(next_antinode.0 as usize, next_antinode.1 as usize)) {
                    anti_nodes.insert((next_antinode.0 as usize, next_antinode.1 as usize));
                }

                delta += 1;
            }
        }
    }
}

pub fn solve_b() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut graph: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        graph.push(line.chars().collect());
    }

    let mut freq_position_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (r, line) in graph.iter().enumerate() {
        for (c, character) in line.iter().enumerate() {
            if character.is_ascii_digit() {
                freq_position_map
                    .entry(*character)
                    .or_default()
                    .push((r, c));
            }

            if character.is_ascii_alphabetic() {
                freq_position_map
                    .entry(*character)
                    .or_default()
                    .push((r, c));
            }
        }
    }

    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, position) in freq_position_map.iter() {
        add_antinodes_resonant(position, &mut anti_nodes, graph.len(), graph[0].len());
    }

    println!("Day 8 B Solution: {}", anti_nodes.len());
}
