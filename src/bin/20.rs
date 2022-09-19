use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Dark,
    Light,
}

struct Grid {
    void: Pixel,
    data: HashMap<(i32, i32), Pixel>,
    dim: (i32, i32),
    lookup: Vec<Pixel>,
}

impl Grid {
    fn read_pixel(&self, pos: (i32, i32)) -> Pixel {
        match self.data.get(&pos) {
            Some(p) => *p,
            None => self.void,
        }
    }

    fn read_pixel_section(&self, pos: (i32, i32)) -> Vec<Pixel> {
        let (x, y) = pos;
        vec![
            self.read_pixel((x - 1, y - 1)),
            self.read_pixel((x, y - 1)),
            self.read_pixel((x + 1, y - 1)),
            self.read_pixel((x - 1, y)),
            self.read_pixel((x, y)),
            self.read_pixel((x + 1, y)),
            self.read_pixel((x - 1, y + 1)),
            self.read_pixel((x, y + 1)),
            self.read_pixel((x + 1, y + 1)),
        ]
    }

    fn read_pixel_section_as_int(&self, pos: (i32, i32)) -> usize {
        self.read_pixel_section(pos)
            .iter()
            .map(|p| match p {
                Pixel::Light => 1,
                Pixel::Dark => 0,
            })
            .fold(0, |acc, i| (acc << 1) | i)
    }

    fn count_light(&self) -> usize {
        self.data.values().filter(|p| **p == Pixel::Light).count()
    }
}

fn get_input() -> Grid {
    let input = fs::read_to_string("inputs/20.txt").unwrap();
    let mut lines = input.lines();
    let lookup = lines.next().unwrap();
    let lookup = lookup
        .chars()
        .map(|c| match c {
            '.' => Pixel::Dark,
            '#' => Pixel::Light,
            _ => panic!("unexpected char: {}", c),
        })
        .collect();
    lines.next();
    let mut grid = HashMap::new();
    let mut m_x = 0i32;
    let mut m_y = 0i32;
    for (y, line) in lines.enumerate() {
        if y as i32 > m_y {
            m_y = y as i32;
        }
        for (x, c) in line.chars().enumerate() {
            if x as i32 > m_x {
                m_x = x as i32;
            }
            match c {
                '.' => {
                    grid.insert((x as i32, y as i32), Pixel::Dark);
                }
                '#' => {
                    grid.insert((x as i32, y as i32), Pixel::Light);
                }
                _ => panic!("unexpected char: {}", c),
            }
        }
    }
    Grid {
        data: grid,
        lookup: lookup,
        void: Pixel::Dark,
        dim: (m_x, m_y),
    }
}

fn main() {
    println!("part1: {}", solve(2));
    println!("part1: {}", solve(50));
}

fn solve(gens: i32) -> usize {
    let mut grid = get_input();
    let (width, height) = grid.dim;

    for i in 1..=gens {
        let mut new_data = HashMap::new();
        for y in -(i + 1)..(height + i + 1) {
            for x in -(i + 1)..(width + i + 1) {
                let new_value = grid.lookup[grid.read_pixel_section_as_int((x, y))];
                new_data.insert((x, y), new_value);
            }
        }
        grid.void = grid.lookup[grid.read_pixel_section_as_int((-100000, -100000))];
        grid.data = new_data;
    }

    grid.count_light()
}
