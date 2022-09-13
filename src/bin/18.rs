use std::fs;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
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
    part2();
}

fn add(left: Box<SNum>, right: Box<SNum>) -> Box<SNum> {
    Box::new(SNum::Pair(left, right))
}

fn explode_in_order<'a>(
    s: &'a mut Box<SNum>,
    depth: u32,
    prev: &mut Option<&'a mut u32>,
    forward_value: &mut Option<u32>,
    modified: &mut bool,
) {
    if depth == 4 && !*modified {
        if let SNum::Pair(left, right) = s.as_ref() {
            if let SNum::Val(left) = left.as_ref() {
                if let Some(prev) = prev {
                    **prev += *left;
                }
            }
            if let SNum::Val(right) = right.as_ref() {
                *forward_value = Some(*right);
            }
            *s.as_mut() = SNum::Val(0);
            *modified = true;
            // return early to avoid applying the forward_value to this 0 value.
            return;
        }
    }
    match s.as_mut() {
        SNum::Val(v) => {
            if let Some(num_value) = forward_value {
                *v += *num_value;
                *forward_value = None;
            }
            *prev = Some(v)
        }
        SNum::Pair(left, right) => {
            explode_in_order(left, depth + 1, prev, forward_value, modified);
            explode_in_order(right, depth + 1, prev, forward_value, modified);
        }
    }
}

fn explode(s: &mut Box<SNum>) -> bool {
    let mut prev: Option<&mut u32> = None;
    let mut forward_value: Option<u32> = None;
    let mut modified = false;
    explode_in_order(s, 0, &mut prev, &mut forward_value, &mut modified);
    modified
}

fn split(s: &mut Box<SNum>) -> bool {
    let replace = match s.as_ref() {
        SNum::Val(v) => *v,
        SNum::Pair(_, _) => 0,
    };
    if replace >= 10 {
        *s.as_mut() = SNum::Pair(
            Box::new(SNum::Val(replace / 2)),
            Box::new(SNum::Val(
                replace / 2 + (if replace % 2 != 0 { 1 } else { 0 }),
            )),
        );
        true
    } else {
        match s.as_mut() {
            SNum::Val(_) => false,
            SNum::Pair(left, right) => split(left) || split(right),
        }
    }
}

fn reduce(s: &mut Box<SNum>) {
    while explode(s) || split(s) {}
}

fn magnitude(s: &Box<SNum>) -> u32 {
    match s.as_ref() {
        SNum::Val(v) => *v,
        SNum::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn part1() {
    let input = get_input();
    let mut pairs = input.into_iter();
    let mut result = pairs.next().unwrap();

    for pair in pairs {
        result = add(result, pair);
        reduce(&mut result);
    }

    println!("part1: {}", magnitude(&result));
}

fn part2() {
    let mut answer = 0;
    for (i, num_a) in get_input().into_iter().enumerate() {
        for (j, num_b) in get_input().into_iter().enumerate() {
            let num_a = num_a.clone();
            if i == j {
                continue;
            }
            let mut result = add(num_a, num_b);
            reduce(&mut result);
            let m = magnitude(&result);
            if m > answer {
                answer = m;
            }
        }
    }
    println!("part2: {}", answer);
}
