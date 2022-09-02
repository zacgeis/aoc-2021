use std::fmt;
use std::fs;
use std::ops::Index;

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
    solve1();
}

fn part1() {
    let mut input = get_input();
    let mut last_number = 0;
    'outer: for number in &input.numbers {
        for board in &mut input.boards {
            board.mark_number(*number);
            if board.complete {
                last_number = *number;
                break 'outer;
            }
        }
    }

    let mut winning_board: Option<&Board> = None;
    for board in &input.boards {
        if board.complete {
            winning_board = Some(board);
        }
    }

    match winning_board {
        None => panic!("No winning board found."),
        Some(board) => {
            let solution = board.unmarked_sum() * last_number;
            println!("part1: {}", solution);
        }
    }
}

// struct Point { x: i32, y: i32 }
//
// fn sample1(p: &mut Point) {
//     let x = &p.x;
//     let y = &p.y;
//
//     println!("x: {}, y: {}, pa.x: {}", x, y, p.x);
// }

// fn sample() {
//     let mut p = Point { x: 1, y: 2 };
//
//     let pa = &mut p;
//     // sample1(pa);
//     let x = &pa.x;
//     let y = &pa.y;
//     pa.x = 1;
//
//     println!("x: {}, y: {}, pa.x: {}", x, y, pa.x);
// }

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

struct Cell(u32, bool);
struct Board {
    width: usize,
    height: usize,
    complete: bool,
    items: Vec<Cell>,
}

fn solve1() {
    let mut input = get_input();
    let mut winning_board: Option<&Board> = None;
    for number in &input.numbers {
        winning_board = find_winning_board1(*number, &mut input.boards);
        match winning_board {
            None => continue,
            Some(_) => break,
        }
    }
    if let Some(board) = winning_board {
        println!("board: {}", board);
    }
}

fn find_winning_board1<'a>(number: u32, boards: &'a mut Vec<Board>) -> Option<&'a Board> {
    for board in boards {
        board.mark_number(number);
        if board.complete {
            return Some(board);
        }
    }
    None
}

// fn solve2() {
//     let mut input = get_input();
//     let winning_board: Option<&Board> = find_winning_board2(&mut input);
//     if let Some(board) = winning_board {
//         println!("board: {}", board);
//     }
// }
//
// fn find_winning_board2<'a>(input: &'a mut Input) -> Option<&'a Board> {
//     for number in &input.numbers {
//         let number = *number;
//         for board in &mut input.boards {
//             board.mark_number(number);
//             if board.complete {
//                 // If None is returned here, there is no issue.
//                 return Some(board);
//             }
//         }
//     }
//     None
// }

// fn part1a() {
//     let mut input = get_input();
//     let mut winning_board: Option<&Board> = None;
//
//     {
//         let mut iter = input.numbers.iter();
//         'outer: loop {
//             let number;
//             match iter.next() {
//                 None => break,
//                 Some(val) => number = val,
//             }
//             println!("number: {}", number);
//             //
//             {
//                 let mut inner_iter = input.boards.iter_mut();
//                 loop {
//                     let board;
//                     match inner_iter.next() {
//                         None => break,
//                         Some(val) => board = val,
//                     }
//
//                     board.mark_number(*number);
//
//                     winning_board = Some(board);
//                     break 'outer;
//                 }
//             }
//             //
//         }
//     }
//
//     if let Some(board) = winning_board {
//         println!("winning_board: {}", board);
//     }
// }
