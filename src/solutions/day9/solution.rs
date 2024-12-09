use std::io::{self, Read};

fn get_memory_layout(input: &str) -> Vec<i64> {
    let mut memory: Vec<i64> = Vec::new();
    let mut mem_id = 0;

    for (i, c) in input.trim_end().chars().enumerate() {
        if i % 2 == 0 {
            let block_len: usize = c.to_string().parse().unwrap();
            memory.extend(vec![mem_id; block_len]);
            mem_id += 1;
        } else {
            let free_len: usize = c.to_string().parse().unwrap();
            memory.extend(vec![-1; free_len]);
        }
    }

    memory
}

fn compact_memory(memory: &mut [i64]) {
    let mut l: usize = 0;
    let mut r: usize = memory.len() - 1;
    while l < r {
        while l < r && memory[l] != -1 {
            l += 1;
        }

        while l < r && memory[r] == -1 {
            r -= 1;
        }

        if l < r {
            memory.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

pub fn solve_a() {
    let mut res: i64 = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut memory = get_memory_layout(&input);
    compact_memory(&mut memory);

    for (i, v) in memory.iter().enumerate() {
        if *v != -1 {
            res += i as i64 * *v;
        }
    }

    println!("Day 9 A Solution: {res}");
}

fn get_block_sizes(input: &str) -> Vec<usize> {
    let mut block_sizes = Vec::new();

    for (i, c) in input.trim_end().chars().enumerate() {
        if i % 2 == 0 {
            let block_len: usize = c.to_string().parse().unwrap();
            block_sizes.push(block_len);
        }
    }

    block_sizes
}

fn get_free_sizes_from_memory(memory: &[i64]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut start_index = None;

    for (i, &value) in memory.iter().enumerate() {
        if value == -1 {
            if start_index.is_none() {
                start_index = Some(i);
            }
        } else if let Some(start) = start_index {
            result.push((start, i - start));
            start_index = None;
        }
    }

    if let Some(start) = start_index {
        result.push((start, memory.len() - start));
    }

    result
}

fn get_block_sizes_with_index(block_sizes: &[usize], memory: &[i64]) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    res.push((0, block_sizes[0]));
    let mut block_idx = 1;

    for i in 1..memory.len() {
        if memory[i] != -1 && memory[i - 1] != memory[i] {
            res.push((i, block_sizes[block_idx]));
            block_idx += 1;
        }
    }

    res
}

pub fn solve_b() {
    let mut res: i64 = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut memory = get_memory_layout(&input);
    let block_sizes = get_block_sizes(&input);
    let mut free_sizes = get_free_sizes_from_memory(&memory);
    let blocks = get_block_sizes_with_index(&block_sizes, &memory);

    for (i, b) in blocks.iter().rev() {
        for (j, (fs, f)) in free_sizes.iter().enumerate() {
            if f >= b && *i > *fs {
                for k in 0..*b {
                    memory.swap(i + k, fs + k);
                }
            }
        }

        free_sizes = get_free_sizes_from_memory(&memory);
    }

    for (i, v) in memory.iter().enumerate() {
        if *v != -1 {
            res += i as i64 * *v;
        }
    }

    println!("Day 9 B Solution: {res}");
}
