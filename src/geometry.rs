use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Display, ops::{Add, Sub}};
use std::hash::Hash;

pub struct CharMap {
    map: HashMap<Vector2, char>,
    specials: HashMap<char, Vector2>,
    obstacle: char,
    free_space: char,
    x_max: usize,
    y_max: usize,
}

impl CharMap {
    pub fn from_string_vec(string_vec: &Vec<String>, obstacle: char, free_space: char) -> Self {
        let mut map = HashMap::new();
        let mut specials = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;

        let mut y = 0;
        for row in string_vec {
            let mut x = 0;
            for c in row.chars() {
                map.insert(Vector2::new(x, y), c);
                if c != obstacle && c != free_space {
                    specials.insert(c, Vector2::new(x, y));
                }
                x += 1;
            }
            x_max = x as usize;
            y += 1;
        }
        y_max = y as usize;
        CharMap { map, specials, obstacle, free_space, x_max, y_max }
    }

    pub fn in_bounds(&self, location: &Vector2) -> bool {
        if location.x >= 0 && location.x < self.x_max as i64 && location.y >= 0 && location.y < self.y_max as i64 {
            return self.map.get(location).unwrap() == &self.free_space;
        }
        return false; 
    }
}

impl Display for CharMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.y_max as i64 {
            for x in 0..self.x_max as i64 {
                s.push(*self.map.get(&Vector2::new(x, y)).unwrap());
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

impl Graph for CharMap {
    type Node = Vector2;

    fn get_neighs(&self, node: &Self::Node) -> Vec<Self::Node> {
        return node.orthogonal_neighs().into_iter().filter(|n| self.in_bounds(n)).collect::<Vec<_>>();
    }
}

pub trait Graph {
    type Node: Eq + Hash + Clone;

    fn get_neighs(&self, node: &Self::Node) -> Vec<Self::Node>;

    fn bfs(&self, start: Self::Node, end: Self::Node) -> Option<u32> {
        let mut queue = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();
        while !queue.is_empty() {
            let (current, steps) = queue.pop_front().unwrap();
            if current == end {
                return Some(steps);
            }
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            for neigh in self.get_neighs(&current) {
                queue.push_back((neigh, steps + 1));
            }
        }
        return None;
    }
}


#[derive(Hash, Clone)]
pub struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Self {
        Vector2{x, y}
    }
    
    pub fn scale(&mut self, scalar: i64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn orthogonal_neighs(&self) -> Vec<Vector2>  {
        vec![
            Vector2::new(self.x + 1, self.y), 
            Vector2::new(self.x - 1, self.y), 
            Vector2::new(self.x, self.y + 1), 
            Vector2::new(self.x, self.y - 1)
            ]
    }

    pub fn diagonal_neighs(&self) -> Vec<Self> {
        vec![
            Vector2::new(self.x + 1, self.y + 1), 
            Vector2::new(self.x - 1, self.y + 1), 
            Vector2::new(self.x + 1, self.y - 1), 
            Vector2::new(self.x - 1, self.y - 1)
            ]
    }

    pub fn length_squared(&self) -> i64 {
        self.x.pow(2) + self.y.pow(2)
    }

    pub fn manhattan(&self, other: &Vector2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vector2 { }

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => None,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => None,

            (std::cmp::Ordering::Less, _) => Some(std::cmp::Ordering::Less),
            (_, std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Less),
            
            (_, std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Greater),
            (std::cmp::Ordering::Greater, _) => Some(std::cmp::Ordering::Greater),
            
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}