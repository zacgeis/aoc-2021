use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

impl Segment {
    fn is_hor(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_ver(&self) -> bool {
        self.start.x == self.end.x
    }

    fn slope(&self) -> (i32, i32) {
        let rise = self.end.y - self.start.y;
        let run = self.end.x - self.start.x;
        let a = gcd(rise, run);
        (rise / a, run / a)
    }

    fn interpolate_points(&self) -> Vec<Point> {
        let (rise, run) = self.slope();
        let mut points: Vec<Point> = vec![];
        let mut current_point = self.start;
        points.push(current_point);
        while current_point != self.end {
            current_point.x += run;
            current_point.y += rise;
            points.push(current_point);
        }
        points
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        Ok(Point { x, y })
    }
}

impl FromStr for Segment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let start = Point::from_str(parts.next().unwrap())?;
        let end = Point::from_str(parts.next().unwrap())?;
        Ok(Segment { start, end })
    }
}

fn main() {
    part1();
    part2();
}

fn get_input() -> Vec<Segment> {
    let input = fs::read_to_string("inputs/05.txt").unwrap();
    input
        .lines()
        .map(|line| Segment::from_str(line))
        // uses the FromIter
        .collect::<Result<Vec<Segment>, String>>()
        .unwrap()
}

fn part1() {
    let segments = get_input();
    let segments = segments
        .into_iter()
        .filter(|segment| segment.is_hor() || segment.is_ver())
        .collect::<Vec<Segment>>();

    let mut overlaps: Vec<Point> = vec![];
    let mut grid: HashMap<Point, i32> = HashMap::new();
    for segment in segments {
        for point in segment.interpolate_points() {
            let val = grid.entry(point).or_insert(0);
            *val += 1;
            if *val == 2 {
                overlaps.push(point);
            }
        }
    }

    println!("part1: {}", overlaps.len());
}

fn part2() {
    let segments = get_input();

    let mut overlaps: Vec<Point> = vec![];
    let mut grid: HashMap<Point, i32> = HashMap::new();
    for segment in segments {
        for point in segment.interpolate_points() {
            let val = grid.entry(point).or_insert(0);
            *val += 1;
            if *val == 2 {
                overlaps.push(point);
            }
        }
    }

    println!("part2: {}", overlaps.len());
}
