use std::{fs, str::FromStr};

fn main() {
    part1();
    part2();
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Missing parts.".to_string());
        }
        let direction = parts[0];
        let units = parts[1].parse().unwrap();
        match direction {
            "forward" => Ok(Self::Forward(units)),
            "down" => Ok(Self::Down(units)),
            "up" => Ok(Self::Up(units)),
            _ => Err(format!("Unknown direction: {}", direction)),
        }
    }
}

fn get_input() -> Vec<Command> {
    let input = fs::read_to_string("inputs/02.txt").unwrap();
    let input: Result<Vec<Command>, String> =
        input.lines().map(|line| Command::from_str(line)).collect();
    input.unwrap()
}

fn part1() {
    let input = get_input();

    let mut h = 0;
    let mut d = 0;

    for command in input {
        match command {
            Command::Up(units) => d -= units,
            Command::Down(units) => d += units,
            Command::Forward(units) => h += units,
        }
    }

    println!("part1: {}", h * d);
}

fn part2() {
    let input = get_input();

    let mut a = 0;
    let mut h = 0;
    let mut d = 0;

    for command in input {
        match command {
            Command::Up(units) => a -= units,
            Command::Down(units) => a += units,
            Command::Forward(units) => {
                h += units;
                d += a * units;
            }
        }
    }

    println!("part2: {}", h * d);
}
