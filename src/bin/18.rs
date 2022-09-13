use std::fs;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
fn in_order<'a>(
    s: &'a mut Box<SNum>,
    depth: u32,
    out: &mut Vec<&'a mut u32>,
    target: &mut Option<(u32, u32, usize)>,
) {
    if depth == 4 && *target == None {
        if let SNum::Pair(left, right) = s.as_ref() {
            let a = if let SNum::Val(a) = left.as_ref() {
                *a
            } else {
                0
            };
            let b = if let SNum::Val(b) = right.as_ref() {
                *b
            } else {
                0
            };
            *target = Some((a, b, out.len()));
            *s.as_mut() = SNum::Val(0);
        }
    }
    match s.as_mut() {
        SNum::Val(v) => out.push(v),
        SNum::Pair(left, right) => {
            in_order(left, depth + 1, out, target);
            in_order(right, depth + 1, out, target);
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
    let mut values = vec![];
    let mut target = None;
    in_order(&mut result, 0, &mut values, &mut target);
    // println!("explode: {:?}", try_explode(&mut result, 0));
    println!("part1: {:?}", &values);
}
