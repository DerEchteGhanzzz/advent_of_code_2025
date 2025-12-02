use std::ops::{Add, Mul, Sub};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64, 
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point{x, y}
    } 

    pub fn out_of_bounds(&self, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
        let Point{x, y} = self;
        *x < x_min || x_max < *x || *y < y_min || y_max < *y
    }

    pub fn x(&self) -> i64 {
        let Self {x, y: _} = self;
        *x
    }

    pub fn y(&self) -> i64 {
        let Self {x: _, y} = self;
        *y
    }

    pub fn scale(&self, rhs: i64) -> Self {
        let Point {x: x1, y: y1} = self;
        Self::new(x1*rhs, y1*rhs)
    }

    pub fn get_neighs(&self) -> Vec<Self> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter().map(|d| self.clone() + d.to_point()).collect::<Vec<_>>()
    }

    pub fn get_diagonal_neighs(&self) -> Vec<Self> {
        vec![(1, 1), (-1, 1), (1, -1), (-1, -1)].iter().map(|(i, j)| self.clone() + Point::new(*i, *j)).collect::<Vec<_>>()
    }

    pub fn length_squared(&self) -> i64 {
        self.x().pow(2) + self.y().pow(2)
    }

    pub fn manhattan(&self, other: &Point) -> i64 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Point {x: x1, y: y1} = self;
        let Point {x: x2, y: y2} = rhs;
        Self::new(x1+x2, y1+y2)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let Point {x: x1, y: y1} = self;
        let Point {x: x2, y: y2} = rhs;
        Point::new(x1+x2, y1+y2)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Point {x: x1, y: y1} = self;
        let Point {x: x2, y: y2} = rhs;
        Self::new(x1-x2, y1-y2)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {

    pub fn turn_right(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn to_point(&self) -> Point {
        match self {
            Self::Up => Point::new(0, -1),
            Self::Right => Point::new(1, 0),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
        }
    }

    pub fn get_opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}