use std::collections::HashSet;
use std::fs;

fn get_input() -> (Vec<(i32, i32)>, HashSet<(i32, i32)>) {
    let input = fs::read_to_string("inputs/13.txt").unwrap();
    let mut lines = input.lines();
    let mut folds: Vec<(i32, i32)> = Default::default();
    let mut points: HashSet<(i32, i32)> = Default::default();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(',');
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        points.insert((x, y));
    }
    while let Some(line) = lines.next() {
        let parts = line.split_whitespace().skip(2).next().unwrap();
        let mut parts = parts.split('=');
        let axis = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();
        let fold = match axis {
            "x" => (value, 0),
            "y" => (0, value),
            _ => panic!("unexpected char: {}", axis),
        };
        folds.push(fold);
    }
    (folds, points)
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (folds, points) = get_input();
    let mut transformed_points: HashSet<(i32, i32)> = Default::default();
    for (mut px, mut py) in points {
        for (fx, fy) in folds.iter().cloned().take(1) {
            if fx != 0 && px > fx {
                px = ((px - fx) * -1) + fx
            }
            if fy != 0 && py > fy {
                py = ((py - fy) * -1) + fy
            }
        }
        transformed_points.insert((px, py));
    }
    println!("part1: {}", transformed_points.len());
}

fn part2() {
    let (folds, points) = get_input();
    let mut transformed_points: HashSet<(i32, i32)> = Default::default();
    for (mut px, mut py) in points {
        for (fx, fy) in folds.iter().cloned() {
            if fx != 0 && px > fx {
                px = ((px - fx) * -1) + fx
            }
            if fy != 0 && py > fy {
                py = ((py - fy) * -1) + fy
            }
        }
        transformed_points.insert((px, py));
    }

    let mut mx = 0;
    let mut my = 0;
    for &(px, py) in &transformed_points {
        if px > mx {
            mx = px;
        }
        if py > my {
            my = py;
        }
    }
    let mut display: Vec<Vec<char>> = Default::default();
    for _ in 0..=my {
        let mut line: Vec<char> = vec![];
        for _ in 0..=mx {
            line.push(' ');
        }
        display.push(line);
    }

    for (x, y) in transformed_points {
        display[y as usize][x as usize] = '#';
    }

    println!("part2:");
    for line in display {
        println!("{}", String::from_iter(line));
    }
}
