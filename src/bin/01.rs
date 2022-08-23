use std::fs;
use std::num::ParseIntError;

fn main() {
    part1();
    part2();
}

fn get_input() -> Vec<u32> {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let values: Result<Vec<u32>, ParseIntError> = input.lines().map(str::parse).collect();
    values.unwrap()
}

fn part1() {
    let input = get_input();

    let iter1 = input.iter();
    let iter2 = input.iter().skip(1);
    let total_count = iter1.zip(iter2).filter(|(a, b)| a < b).count();

    println!("part1: {}", total_count);
}

fn part2() {
    let input = get_input();

    let iter1 = input.iter();
    let iter2 = input.iter().skip(3);
    let total_count = iter1.zip(iter2).filter(|(a, b)| a < b).count();

    println!("part2: {}", total_count);
}
