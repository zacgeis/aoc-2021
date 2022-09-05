use std::fs;

fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("inputs/07.txt").unwrap();
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}

fn main() {
    part1();
    part2();
}

fn solve<F>(left_bound: i32, right_bound: i32, f: F) -> i64
where
    F: Fn(i32) -> i64,
{
    let mut left_bound = left_bound;
    let mut right_bound = right_bound;
    loop {
        let mid = (left_bound + right_bound) / 2;
        let mid_cost = f(mid);
        let left_cost = f(mid - 1);
        if left_cost > mid_cost {
            left_bound = mid;
        } else {
            right_bound = mid;
        }
        if left_bound + 1 == right_bound {
            break;
        }
    }
    f(left_bound)
}

fn part1() {
    let positions = get_input();
    let left_bound = *positions.iter().min().unwrap();
    let right_bound = *positions.iter().max().unwrap();
    let solution = solve(left_bound, right_bound, |target| {
        positions
            .iter()
            .map(|position| (target - position).abs() as i64)
            .sum()
    });
    println!("part1: {}", solution);
}

fn cost(i: i64) -> i64 {
    i * (i + 1) / 2
}

fn part2() {
    let positions = get_input();
    let left_bound = *positions.iter().min().unwrap();
    let right_bound = *positions.iter().max().unwrap();
    let solution = solve(left_bound, right_bound, |target| {
        positions
            .iter()
            .map(|position| cost((target - position).abs() as i64))
            .sum()
    });
    println!("part2: {}", solution);
}
