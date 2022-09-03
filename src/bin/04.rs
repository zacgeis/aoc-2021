use std::fmt;
use std::fs;
use std::ops::Index;

struct Cell(u32, bool);
struct Board {
    width: usize,
    height: usize,
    complete: bool,
    items: Vec<Cell>,
}

impl Board {
    fn row_iter<'a>(&'a self, row: usize) -> impl Iterator<Item = &Cell> + 'a {
        (0..self.width).map(move |x| &self[(x, row)])
    }

    fn col_iter<'a>(&'a self, col: usize) -> impl Iterator<Item = &Cell> + 'a {
        (0..self.height).map(move |y| &self[(col, y)])
    }

    fn check_row(&self, row: usize) -> bool {
        self.row_iter(row).fold(true, |acc, cell| acc && cell.1)
    }

    fn check_col(&self, col: usize) -> bool {
        self.col_iter(col).fold(true, |acc, cell| acc && cell.1)
    }

    fn unmarked_sum(&self) -> u32 {
        self.items
            .iter()
            .filter(|cell| !cell.1)
            .map(|cell| cell.0)
            .sum()
    }

    fn mark_number(&mut self, num: u32) {
        let mut marked_locs: Vec<(usize, usize)> = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &mut self.items[x + y * self.width];
                if cell.0 == num {
                    cell.1 = true;
                    marked_locs.push((x, y));
                }
            }
        }

        self.complete = self.complete
            || marked_locs
                .iter()
                .any(|(x, y)| self.check_row(*y) || self.check_col(*x));
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
            complete: false,
            items: items,
        });
    }

    Input { numbers, boards }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = get_input();
    let numbers = input.numbers;
    let mut boards = input.boards;

    let mut winning_board: Option<&Board> = None;
    let mut last_number = 0;
    for number in numbers {
        last_number = number;
        winning_board = find_winning_board(number, &mut boards);
        match winning_board {
            None => continue,
            Some(_) => break,
        }
    }

    if let Some(board) = winning_board {
        println!("part1: {}", board.unmarked_sum() * last_number);
    }
}

// separate function required due to bug in the borrow checker.
fn find_winning_board<'a>(number: u32, boards: &'a mut Vec<Board>) -> Option<&'a Board> {
    for board in boards {
        board.mark_number(number);
        if board.complete {
            return Some(board);
        }
    }
    None
}

fn part2() {
    let input = get_input();
    let numbers = input.numbers;
    let mut boards = input.boards;

    let mut last_number = 0;
    let mut last_winning_board: Option<Board> = None;
    for number in numbers {
        last_number = number;
        let mut remaining_boards: Vec<Board> = vec![];
        for mut board in boards {
            board.mark_number(number);
            if board.complete {
                last_winning_board = Some(board);
                continue;
            }
            remaining_boards.push(board);
        }
        if remaining_boards.len() == 0 {
            break;
        }
        boards = remaining_boards;
    }

    match last_winning_board {
        None => panic!("Winning board not found."),
        Some(board) => {
            println!("part2: {}", board.unmarked_sum() * last_number);
        }
    }
}
