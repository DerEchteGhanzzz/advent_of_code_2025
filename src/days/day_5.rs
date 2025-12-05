#![allow(unused)]

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Range {
    begin: i64,
    end: i64,
}

impl Range {
    fn includes(&self, x: i64) -> bool {
        self.begin <= x && x <= self.end
    }

    fn includes_range(&self, other: &Range) -> bool {
        (self.begin <= other.begin && other.begin <= self.end) ||
        (other.begin <= self.begin && self.begin <= other.end)
    }

    fn combine_ranges(&self, other: &Range) -> Vec<Range> {
        if !&self.includes_range(other) {
            if self.begin < other.begin {
                return vec![self.clone(), other.clone()];
            }
            return vec![other.clone(), self.clone()];
        }
        
        return vec![Range {begin: self.begin.min(other.begin), end: self.end.max(other.end)}]
    }
}

fn parse_ranges(input: Vec<&String>) -> Vec<Range> {
    input.into_iter().map(|s| {
        let mut split = s.split("-");
        let fst = split.next().unwrap();
        let snd = split.next().unwrap();
        Range {begin: i64::from_str_radix(fst, 10).unwrap(), end: i64::from_str_radix(snd, 10).unwrap()}
    }).collect::<Vec<_>>()
}

fn parse_input(input: &Vec<String>) -> (Vec<Range>, Vec<i64>) {
    let mut split = input.split(|s| s == "");
    let fst = split.next().unwrap().into_iter().collect::<Vec<_>>();
    let snd = split.next().unwrap().into_iter().map(|x| i64::from_str_radix(x, 10).unwrap()).collect::<Vec<_>>();
    (parse_ranges(fst), snd)
}

pub fn solve_a(input: &Vec<String>) -> usize {
    let (ranges, storage) = parse_input(input);
    storage.iter().filter(|x| ranges.iter().any(|r| r.includes(**x))).count()
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let (mut ranges, _) = parse_input(input);
    ranges.sort();
    let mut total = 0;
    let mut prev_end = 0;
    for range in ranges {
        let mut subtotal = 0;
        if range.includes(prev_end) {
            subtotal = range.end - prev_end;
        }  else if (range.end <= prev_end) {
            subtotal = 0;
        } else if (prev_end < range.begin) {
            subtotal = range.end - range.begin + 1;
        }
        total += subtotal;
        prev_end = prev_end.max(range.end);
    }
    total
}
// too high 351192334583375