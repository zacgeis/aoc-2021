use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

struct Grid {
    width: isize,
    height: isize,
    data: Vec<u16>,
}

impl Grid {
    fn unbounded_get(&self, scale: u8, pos: (isize, isize)) -> u16 {
        let scaled_width = scale as isize * self.width;
        let scaled_height = scale as isize * self.height;
        let (x, y) = pos;
        if x >= 0 && x < scaled_width && y >= 0 && y < scaled_height {
            let scalar = (x / self.width + y / self.height) as u16;
            let value = self.data[(self.width * (y % self.height) + (x % self.width)) as usize];
            let mut scaled_value = value + scalar;
            if scaled_value > 9 {
                scaled_value %= 9;
            }
            scaled_value
        } else {
            u16::MAX
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Path {
    cost: u32,
    pos: (isize, isize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_input() -> Grid {
    let input = fs::read_to_string("inputs/15.txt").unwrap();
    let mut data = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        for cell in line.chars() {
            data.push(cell.to_digit(10).unwrap() as u16);
        }
        if width == 0 {
            width = line.chars().count() as isize;
        }
        height += 1;
    }
    Grid {
        width,
        height,
        data,
    }
}

fn main() {
    part1();
    part2();
}

fn solve(grid: Grid, scale: u8) -> u32 {
    let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();

    let start_pos = (0, 0);
    let end_pos = (
        (grid.width * scale as isize) - 1,
        (grid.height * scale as isize) - 1,
    );
    heap.push(Path {
        cost: 0,
        pos: start_pos,
    });

    let mut lowest_cost = 0;
    while let Some(current) = heap.pop() {
        if visited.contains(&current.pos) {
            continue;
        }
        visited.insert(current.pos);
        if current.pos == end_pos {
            lowest_cost = current.cost;
            break;
        }
        for (offset_x, offset_y) in neighbors {
            let (x, y) = current.pos;
            let next_pos = (x + offset_x, y + offset_y);
            let next_cost = current.cost + grid.unbounded_get(scale, next_pos) as u32;
            heap.push(Path {
                cost: next_cost,
                pos: next_pos,
            });
        }
    }
    lowest_cost
}

fn part1() {
    let grid = get_input();
    println!("part1: {}", solve(grid, 1));
}

fn part2() {
    let grid = get_input();
    println!("part2: {}", solve(grid, 5));
}
