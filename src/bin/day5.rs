use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::LineSegment::{DiagLine, HorizLine, VertLine};

// this is messy, bad code.
// but it works and i'm too lazy to do it correctly at 10pm

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("could not read input");
    println!("Part 1");
    part1(&buffer);
    println!("Part 2");
    part2(&buffer);
}

// === shared ===
struct Point(u32, u32);

impl Point {
    fn from_str(str: &str) -> Point {
        let (a, b) = str.split_once(",").expect("invalid point");
        Point(a.parse::<u32>().expect("invalid int"), b.parse::<u32>().expect("invalid int"))
    }
}

enum LineSegment {
    VertLine {
        x: u32,
        y1: u32,
        y2: u32,
    },
    HorizLine {
        x1: u32,
        x2: u32,
        y: u32,
    },
    DiagLine {
        x1: u32,
        x2: u32,
        y1: u32,
        y2: u32,
    },
}


impl LineSegment {
    fn from_str(str: &str) -> LineSegment {
        let (start, end) = str.split_once(" -> ").expect("invalid linesegment");
        match (Point::from_str(start), Point::from_str(end)) {
            (Point(x1, y1), Point(x2, y2)) if x1 == x2 => VertLine { x: x1, y1, y2 },
            (Point(x1, y1), Point(x2, y2)) if y1 == y2 => HorizLine { x1, x2, y: y1 },
            (Point(x1, y1), Point(x2, y2)) if (x2 as i32 - x1 as i32).abs() == (y2 as i32 - y1 as i32).abs() => DiagLine { x1, x2, y1, y2 },
            _ => panic!("help idk lines")
        }
    }
}

// this isn't the most efficient implementation, but i'm too lazy to do math tonight
struct Grid(Vec<Vec<u32>>);

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for col in &self.0 {
            for row in col {
                if *row == 0 { write!(f, ".")? } else { write!(f, "{}", row)? }
            }
            writeln!(f)?
        }
        writeln!(f)
    }
}

impl Grid {
    fn init() -> Grid {
        Grid(vec![vec![0; 1000]; 1000])
    }
}

fn parse_input(input: &String) -> Vec<LineSegment> {
    input
        .split("\n")
        .map(|line| LineSegment::from_str(line))
        .collect()
}

// === p1 ===
fn part1(input: &String) {
    let segments = parse_input(input);
    let mut grid = Grid::init();
    // let's write to grid, wahey
    for segment in segments {
        match segment {
            VertLine { x, y1, y2 } => {
                for y in min(y1, y2)..=max(y1, y2) {
                    grid.0[x as usize][y as usize] += 1;
                }
            }
            HorizLine { x1, x2, y } => {
                for x in min(x1, x2)..=max(x1, x2) {
                    grid.0[x as usize][y as usize] += 1;
                }
            }
            _ => {}
        }
    }

    // println!("{}", grid);

    // and find all the points that are >= 1
    let mut result = 0;
    for col in grid.0 {
        for row in col {
            if row > 1 {
                result += 1;
            }
        }
    }
    println!("{}", result)
}

fn part2(input: &String) {
    let segments = parse_input(input);
    let mut grid = Grid::init();
    // let's write to grid, wahey
    for segment in segments {
        match segment {
            VertLine { x, y1, y2 } => {
                for y in min(y1, y2)..=max(y1, y2) {
                    grid.0[x as usize][y as usize] += 1;
                }
            }
            HorizLine { x1, x2, y } => {
                for x in min(x1, x2)..=max(x1, x2) {
                    grid.0[x as usize][y as usize] += 1;
                }
            }
            DiagLine { x1, x2, y1, y2 } => {
                let x_iter: Vec<u32>;
                let y_iter: Vec<u32>;
                if x2 < x1 { x_iter = (x2..=x1).rev().collect() } else { x_iter = (x1..=x2).collect() }
                if y2 < y1 { y_iter = (y2..=y1).rev().collect() } else { y_iter = (y1..=y2).collect() }
                for (&x, &y) in (x_iter.iter()).zip(y_iter.iter()) {
                    grid.0[x as usize][y as usize] += 1;
                }
            }
        }
    }

    // println!("{}", grid);

    // and find all the points that are >= 1
    let mut result = 0;
    for col in grid.0 {
        for row in col {
            if row > 1 {
                result += 1;
            }
        }
    }
    println!("{}", result)
}
