pub fn solve_a(input: &Vec<String>) -> i64 {
    input[0].split(",").map(|s| 
        {
            let mut splt = s.split("-");
            let b = splt.next().unwrap();
            let e = splt.next().unwrap();
            i64::from_str_radix(b, 10).unwrap()..=i64::from_str_radix(e, 10).unwrap()
        }).flat_map(|it| it.filter(|x| !check_valid(x))).sum()
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    input[0].split(",").map(|s| 
        {
            let mut splt = s.split("-");
            let b = splt.next().unwrap();
            let e = splt.next().unwrap();
            i64::from_str_radix(b, 10).unwrap()..=i64::from_str_radix(e, 10).unwrap()
        }).flat_map(|it| it.filter(|x| !check_valid2(x))).sum()
}

fn check_valid(x: &i64) -> bool {
    let s = x.to_string();
    if s.chars().next() == Some('0') {
        return false;
    }
    s[0..s.len()/2] != s[s.len()/2..s.len()]
}

fn check_valid2(x: &i64) -> bool {
    let s = x.to_string();
    if s.len() == 1 {
        return true;
    }
    if s.chars().next() == Some('0') {
        return false;
    }
    for j in (1..=s.len() / 2).rev() {
        
        if s.len() % j != 0 {
            continue;
        } 

        let mut invalid = true;
        let ss = &s[0..j];
        for i in (j..=s.len() - j).step_by(j) {
            invalid = invalid && ss == &s[i..i+j];
        }
        if invalid {
            return false;
        }
    }
    true
}