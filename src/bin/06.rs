use std::fs;

fn get_input() -> Vec<u8> {
    let input = fs::read_to_string("inputs/06.txt").unwrap();
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

fn solve(days: u32) -> u64 {
    let values = get_input();
    let mut sim: [u64; 9] = Default::default();
    for value in values {
        sim[value as usize] += 1;
    }
    for _ in 0..days {
        let expired = sim[0];
        for i in 1..=8 {
            sim[i - 1] = sim[i];
        }
        sim[6] += expired;
        sim[8] = expired;
    }
    sim.iter().sum::<u64>()
}

fn part1() {
    println!("part1: {}", solve(80));
}

fn part2() {
    println!("part2: {}", solve(256));
}
