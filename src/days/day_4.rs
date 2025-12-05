#![allow(unused)]

use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> (HashMap<(usize, usize), char>, usize, usize) {
    let mut map = HashMap::new();
    let mut y = 0;
    let mut x_max = 0;
    let mut y_max = 0;
    
    for row in input {
        let mut x = 0;
        for c in row.chars() {
            map.insert((x, y), c);
            x += 1;
            x_max = x_max.max(x);
        }
        y += 1;
        y_max = y_max.max(y);
    }
    (map, x_max, y_max)
}

fn get_neighs(pos: &(usize, usize), x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let (x, y) = *pos;
    let mut v = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            if (i, j) == (0, 0) { continue; }
            if (x as i32 + i) < 0 || (x as i32 + i) >= x_max as i32 || (y as i32 + j) < 0 || (y as i32 + j) >= y_max as i32 {
                continue;
            }
            v.push(((x as i32 + i) as usize, (y as i32 +j) as usize));
        }
    }
    v
}

fn get_reachable_rolls(m: &HashMap<(usize, usize), char>, x_max: usize, y_max: usize) -> Vec<(usize, usize)>{
    m.clone().into_keys().filter(|(pos)|
        m.get(pos) == Some(&'@') &&
        get_neighs(pos, x_max, y_max).into_iter().flat_map(|neigh| m.get(&neigh)).filter(|c| **c == '@').count() < 4 
    ).collect::<Vec<_>>()
}

pub fn solve_a(input: &Vec<String>) -> usize {
    let (m, x_max, y_max) = parse_input(input);
    get_reachable_rolls(&m, x_max, y_max).len()
}

pub fn solve_b(input: &Vec<String>) -> usize {
    let (mut m, x_max, y_max) = parse_input(input);
    let mut total = 0;
    loop {
        let reachables = get_reachable_rolls(&m, x_max, y_max);
        if reachables.len() == 0 {
            break
        }
        total += reachables.len();
        for r in reachables {
            m.remove(&r);
        }
    }
    total
}