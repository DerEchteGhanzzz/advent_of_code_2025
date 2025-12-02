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
        }).flat_map(|it| it.filter(|x| is_invalid(x))).sum()
}

fn check_valid(x: &i64) -> bool {
    let s = x.to_string();
    s[0..s.len()/2] != s[s.len()/2..s.len()]
}

fn is_invalid(x: &i64) -> bool {
    let s = x.to_string();
    
    for i in (2..=s.len() / 2).filter(|i| s.len() % i == 0) {
        let ss = &s[0..s.len()/i];
        let mut invalid = true;
        for j in (s.len()/i..s.len()).step_by(s.len()/i) {
            if ss != &s[j..j+s.len()/i] {
                invalid = false;
                break;
            }
        }
        if invalid {
            return true;
        }
    }
    s.chars().all(|c| c.to_string() == s[0..1])
}