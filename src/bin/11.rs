use std::fs;

struct Grid {
    data: Vec<i32>,
    width: isize,
    height: isize,
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Grid {
    fn step(&mut self) -> u32 {
        let mut total = 0;

        self.data.iter_mut().for_each(|v| *v += 1);

        'finished: loop {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = (x, y);
                    let cell = self.data[(x + y * self.width) as usize];
                    if cell > 9 {
                        total += 1;
                        self.flash(pos);
                        continue 'finished;
                    }
                }
            }
            break 'finished;
        }

        self.data.iter_mut().for_each(|v| {
            if *v == -1 {
                *v = 0
            }
        });

        total
    }

    fn flash(&mut self, pos: (isize, isize)) {
        let (x, y) = pos;
        let neighbor_positions: Vec<(isize, isize)> = NEIGHBORS
            .iter()
            .map(|(offset_x, offset_y)| (offset_x + x, offset_y + y))
            .filter(|(n_x, n_y)| *n_x >= 0 && *n_x < self.width && *n_y >= 0 && *n_y < self.height)
            .collect();
        self.data[(x + y * self.width) as usize] = -1;
        for neighbor_position in neighbor_positions {
            let (n_x, n_y) = neighbor_position;
            let cell = &mut self.data[(n_x + n_y * self.width) as usize];
            if *cell != -1 {
                *cell += 1;
            }
        }
    }
}

fn get_input() -> Grid {
    let input = fs::read_to_string("inputs/11.txt").unwrap();
    let mut data: Vec<i32> = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        for c in line.chars() {
            data.push(c.to_digit(10).unwrap() as i32);
        }
        if width == 0 {
            width = line.len() as isize;
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
    let mut grid = get_input();
    let mut total = 0;
    for _ in 0..100 {
        total += grid.step();
    }
    println!("part1: {}", total);
}

fn part2() {
    let mut grid = get_input();
    let mut step = 0;
    loop {
        step += 1;
        if grid.step() == grid.data.len() as u32 {
            break;
        }
    }
    println!("part2: {}", step);
}
