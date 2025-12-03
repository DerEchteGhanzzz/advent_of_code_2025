#![allow(unused)]

use std::collections::HashMap;

pub fn solve_a(input: &Vec<String>) -> i64 {
    input.into_iter().map(get_joltage).sum()
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    input.into_iter().map(|s|
        get_large_joltage(s, 0, 0, &mut HashMap::new())).sum()
}

fn get_joltage(bank: &String) -> i64{
    let mut joltage = 0;
    for (i, tens) in bank.chars().enumerate() {
        for ones in bank.chars().skip(i + 1) {
            let n = i64::from_str_radix(&String::from_iter(vec![tens, ones]), 10).unwrap();
            joltage = joltage.max(n);
        }
    }
    return joltage
}

fn get_large_joltage(bank: &String, start: usize, count: i64, mem: &mut HashMap<(usize, i64), i64>) -> i64 {
    
    if mem.contains_key(&(start, count)) {
        return *mem.get(&(start, count)).unwrap();
    }
    if bank.len() - start == count as usize - 1 {
        let d = i64::from_str_radix(&bank[start..], 10).unwrap();
        mem.insert((start, count), d);
        return d;
    }

    if count == 11 || start == bank.len() - 1 {
        let d = bank.chars().skip(start).map(|c| i64::from_str_radix(&c.to_string(), 10).unwrap()).max().unwrap();
        mem.insert((start, count), d);
        return d;
    }
    
    let mut joltage = 0;
    for (i, digit) in bank.chars().enumerate().skip(start) {
        let rest = get_large_joltage(bank, i + 1, count + 1, mem).to_string();
        let n = i64::from_str_radix(&(digit.to_string() + &rest), 10).unwrap();
        joltage = joltage.max(n);
    }
    mem.insert((start, count), joltage);
    return joltage
}