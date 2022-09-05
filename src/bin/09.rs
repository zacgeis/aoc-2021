use std::fs;

struct Grid {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Grid {
    fn unbounded_get(&self, pos: (i32, i32)) -> u8 {
        let (x, y) = pos;
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.data[(x + y * self.width) as usize]
        } else {
            9
        }
    }

    fn is_low_point(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        let pos_value = self.unbounded_get(pos);

        let neighbors = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
        !neighbors
            .iter()
            .map(|pos| self.unbounded_get(*pos))
            .any(|neighbor_value| neighbor_value <= pos_value)
    }

    fn visited(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.data[(x + y * self.width) as usize] = 9;
    }
}

fn get_input() -> Grid {
    let input = fs::read_to_string("inputs/09.txt").unwrap();
    let mut height = 0;
    let mut width = 0;
    let mut data: Vec<u8> = vec![];
    for line in input.lines() {
        for c in line.chars() {
            data.push(c.to_digit(10).unwrap() as u8);
        }
        if width == 0 {
            width = data.len() as i32;
        }
        height += 1;
    }
    Grid {
        data,
        width,
        height,
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = get_input();
    let mut total: u64 = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = (x, y);
            if grid.is_low_point(pos) {
                let risk_level = grid.unbounded_get(pos) + 1;
                total += risk_level as u64;
            }
        }
    }
    println!("part1: {}", total);
}

fn explore_basin(grid: &mut Grid, pos: (i32, i32)) -> u64 {
    let value = grid.unbounded_get(pos);
    if value == 9 {
        0
    } else {
        let (x, y) = pos;
        grid.visited(pos);
        let up = explore_basin(grid, (x, y - 1));
        let down = explore_basin(grid, (x, y + 1));
        let left = explore_basin(grid, (x - 1, y));
        let right = explore_basin(grid, (x + 1, y));
        up + down + left + right + 1
    }
}

fn part2() {
    let mut grid = get_input();
    let mut sizes: Vec<u64> = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = (x, y);
            let basin_size = explore_basin(&mut grid, pos);
            if basin_size > 0 {
                sizes.push(basin_size);
            }
        }
    }
    sizes.sort();
    let total = sizes.iter().rev().take(3).fold(1, |acc, s| acc * s);
    println!("part2: {:?}", total);
}
