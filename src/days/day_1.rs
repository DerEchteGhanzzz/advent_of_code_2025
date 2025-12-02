pub fn solve_a(input: &Vec<String>) -> usize {
    let mut zeros = 0;
    let mut pointer = 50;
    for rotation in input.into_iter().map(|l| if &l[0..1] == "L" { -1 } else { 1 } * i64::from_str_radix(&l[1..], 10).unwrap()) {
        pointer += rotation;
        pointer = ((pointer % 100) + 100) % 100;
        if pointer == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let mut zeros = 0;
    let mut pointer = 50;
    for rotation in input.into_iter().map(|l| if &l[0..1] == "L" { -1 } else { 1 } * i64::from_str_radix(&l[1..], 10).unwrap()) {
        pointer += rotation;
        pointer = ((pointer % 100) + 100) % 100;
        if pointer - rotation >= 100 {
            zeros += ((pointer - rotation) / 100).abs();
        } else if pointer - rotation <= 0 {
            zeros += ((pointer - rotation) / 100).abs() + if pointer == 0 { 0 } else { 1 };
        }
    }
    zeros
}