use std::fmt;
use std::fs;
use std::ops::Index;

struct Cell(u32, bool);
struct Board {
    width: usize,
    height: usize,
    items: Vec<Cell>,
}

impl Board {
    fn row_iter<'a>(&'a self, row: usize) -> impl Iterator<Item = &Cell> + 'a {
        (0..self.width).map(move |x| &self[(x, row)])
    }

    fn col_iter<'a>(&'a self, col: usize) -> impl Iterator<Item = &Cell> + 'a {
        (0..self.height).map(move |y| &self[(col, y)])
    }

    fn mark_number(num: u32) -> bool {
        // TODO: sweep and mark the number in items.
        // check the row and column after setting the value to see if the board is complete.
        false
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.items[x + y * self.width]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self[(x, y)];
                if cell.1 {
                    write!(f, "\x1b[1m")?;
                }
                write!(f, "{: >2} ", cell.0)?;
                if cell.1 {
                    write!(f, "\x1b[0m")?;
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

fn get_input() -> Input {
    let input = fs::read_to_string("inputs/04.txt").unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let numbers = first_line
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Board> = vec![];
    lines.next();
    'outer: loop {
        let mut items: Vec<Cell> = vec![];
        loop {
            let line = lines.next();
            match line {
                None => break 'outer,
                Some("") => break,
                Some(line) => {
                    line.split_whitespace()
                        .map(|number| number.parse().unwrap())
                        .for_each(|number| items.push(Cell(number, false)));
                }
            }
        }
        boards.push(Board {
            width: 5,
            height: 5,
            items: items,
        });
    }

    Input { numbers, boards }
}

fn main() {
    part1();
}

fn part1() {
    let input = get_input();
    let board1 = &input.boards[0];
    println!("{}", board1);
}
