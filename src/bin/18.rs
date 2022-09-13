use std::fs;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
enum SNum {
    Pair(Box<SNum>, Box<SNum>),
    Val(u32),
}

fn parse_snum(tokens: &mut Peekable<impl Iterator<Item = char>>) -> Box<SNum> {
    if *tokens.peek().unwrap() == '[' {
        tokens.next();
        let left = parse_snum(tokens);
        if tokens.next().unwrap() != ',' {
            panic!("expected ,");
        }
        let right = parse_snum(tokens);
        if tokens.next().unwrap() != ']' {
            panic!("expected ]");
        }
        Box::new(SNum::Pair(left, right))
    } else {
        let mut digits = vec![];
        while tokens.peek().unwrap().is_numeric() {
            digits.push(tokens.next().unwrap());
        }
        let value = digits.iter().collect::<String>().parse().unwrap();
        Box::new(SNum::Val(value))
    }
}

fn get_input() -> Vec<Box<SNum>> {
    let input = fs::read_to_string("inputs/18.txt").unwrap();
    input
        .lines()
        .map(|line| {
            let mut peek = line.chars().peekable();
            parse_snum(&mut peek)
        })
        .collect()
}

fn main() {
    part1();
}

fn add(left: Box<SNum>, right: Box<SNum>) -> Box<SNum> {
    Box::new(SNum::Pair(left, right))
}

// find node to explode.
// then do a pre-order and post-order traversal to add the right and left values.
// then update the node to be 0.
fn try_explode(s: &mut Box<SNum>, depth: u8) -> (Option<u32>, Option<u32>, bool) {
    match s.as_mut() {
        SNum::Val(_) => (None, None, false),
        SNum::Pair(left, right) => {
            if depth == 4 {
                let mut left_value: Option<u32> = None;
                let mut right_value: Option<u32> = None;
                if let SNum::Val(value) = left.as_ref() {
                    left_value = Some(*value);
                }
                if let SNum::Val(value) = right.as_ref() {
                    right_value = Some(*value);
                }
                *s.as_mut() = SNum::Val(0);
                (left_value, right_value, true)
            } else {
                let (mut left_value, mut right_value, mut exploded) = try_explode(left, depth + 1);
                if left_value != None || right_value != None {
                    if let SNum::Val(value) = right.as_mut() {
                        if let Some(v) = right_value {
                            *value += v;
                            right_value = None;
                        }
                    }
                } else {
                    (left_value, right_value, exploded) = try_explode(right, depth + 1);
                    if left_value != None || right_value != None {
                        if let SNum::Val(value) = left.as_mut() {
                            if let Some(v) = left_value {
                                *value += v;
                                left_value = None;
                            }
                        }
                    }
                }
                (left_value, right_value, exploded)
            }
        }
    }
}

fn reduce(s: &mut Box<SNum>) {}

fn part1() {
    let input = get_input();
    let mut pairs = input.into_iter();
    let mut result = pairs.next().unwrap();
    // for pair in pairs {}
    println!("pre: {:?}", result);
    println!("explode: {:?}", try_explode(&mut result, 0));
    println!("part1: {:?}", result);
}
