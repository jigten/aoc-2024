use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

trait IntoMiddle {
    fn get_middle(&self) -> i32;
}

impl IntoMiddle for &str {
    fn get_middle(&self) -> i32 {
        let updates: Vec<&str> = self.split(',').collect();
        let middle_update = updates[updates.len() / 2];
        middle_update.parse().unwrap()
    }
}

impl IntoMiddle for &Vec<i32> {
    fn get_middle(&self) -> i32 {
        self[self.len() / 2]
    }
}

fn build_after_hashmap(rules: &str) -> HashMap<i32, Vec<i32>> {
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in rules.lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x_n: i32 = x.parse().unwrap();
        let y_n: i32 = y.parse().unwrap();

        after.entry(x_n).or_insert_with(Vec::new).push(y_n);
    }
    after
}

pub fn solve_a() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let after = build_after_hashmap(rules);

    for upd in updates.lines() {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut is_valid = true;

        for n in upd.split(',') {
            let v: i32 = n.parse().unwrap();

            if let Some(afters) = after.get(&v) {
                for a in afters {
                    if seen.contains(a) {
                        is_valid = false;
                        break;
                    }
                }
            }

            seen.insert(v);
        }

        if is_valid {
            res += upd.get_middle();
        }
    }

    println!("Day 5 A Solution: {res}");
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32], after: &HashMap<i32, Vec<i32>>) {
    let mut l = 0;
    let mut r = 0;
    let mut m = 0;

    while l < left.len() && r < right.len() {
        if let Some(afters) = after.get(&right[r]) {
            if afters.contains(&left[l]) {
                merged[m] = right[r];
                r += 1;
                m += 1;
            } else {
                merged[m] = left[l];
                l += 1;
                m += 1;
            }
        } else {
            merged[m] = left[l];
            l += 1;
            m += 1;
        }
    }

    while l < left.len() {
        merged[m] = left[l];
        l += 1;
        m += 1;
    }

    while r < right.len() {
        merged[m] = right[r];
        r += 1;
        m += 1;
    }
}

fn get_fixed_update(upd: &mut [i32], after: &HashMap<i32, Vec<i32>>) {
    let len = upd.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = upd.split_at_mut(mid);

    get_fixed_update(left, after);
    get_fixed_update(right, after);

    let mut merged = vec![0; len];
    merge(left, right, &mut merged, after);

    upd.copy_from_slice(&merged);
}

pub fn solve_b() {
    let mut res = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let after = build_after_hashmap(rules);

    for upd in updates.lines() {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut is_valid = true;

        for n in upd.split(',') {
            let v: i32 = n.parse().unwrap();

            if let Some(afters) = after.get(&v) {
                for a in afters {
                    if seen.contains(a) {
                        is_valid = false;
                        break;
                    }
                }
            }

            seen.insert(v);
        }

        if !is_valid {
            let mut upd_vec: Vec<i32> = upd
                .split(",")
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            get_fixed_update(&mut upd_vec, &after);
            res += (&upd_vec).get_middle();
        }
    }

    println!("Day 5 B Solution: {res}");
}
