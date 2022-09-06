use std::collections::HashMap;
use std::fs;

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    part1();
    part2();
}

fn get_mapping() -> HashMap<char, char> {
    let mut mapping: HashMap<char, char> = HashMap::new();

    mapping.insert(')', '(');
    mapping.insert('}', '{');
    mapping.insert(']', '[');
    mapping.insert('>', '<');

    mapping
}

fn part1() {
    let input = get_input();

    let mapping = get_mapping();
    let mut points: HashMap<char, i32> = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut total = 0;
    for line in input {
        let mut stack: Vec<char> = vec![];
        for c in line {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' | '}' | ']' | '>' => match stack.pop() {
                    Some(top) => {
                        let expected = mapping[&c];
                        if top != expected {
                            total += points[&c];
                            break;
                        }
                    }
                    None => break,
                },
                c => panic!("unexpected char found: {}", c),
            }
        }
    }

    println!("part1: {}", total);
}

fn part2() {
    let input = get_input();

    let mapping = get_mapping();
    let mut points: HashMap<char, i32> = HashMap::new();
    points.insert('(', 1);
    points.insert('[', 2);
    points.insert('{', 3);
    points.insert('<', 4);

    let mut scores: Vec<i64> = vec![];
    for line in input {
        let mut stack: Vec<char> = vec![];
        for c in line {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' | '}' | ']' | '>' => match stack.pop() {
                    Some(top) => {
                        let expected = mapping[&c];
                        if top != expected {
                            stack.clear();
                            break;
                        }
                    }
                    // left incomplete apparently never happens.
                    None => break,
                },
                c => panic!("unexpected char found: {}", c),
            }
        }
        if !stack.is_empty() {
            let mut total: i64 = 0;
            while let Some(c) = stack.pop() {
                total *= 5;
                total += points[&c] as i64;
            }
            scores.push(total);
        }
    }

    scores.sort();
    let total = scores[scores.len() / 2];
    println!("part2: {}", total);
}
