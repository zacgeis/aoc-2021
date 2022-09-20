use std::fs;
use std::str::FromStr;

enum Op {
    On,
    Off,
}

struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl FromStr for Cuboid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct RebootCommand {
    op: Op,
    cuboid: Cuboid,
}

impl FromStr for RebootCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn get_input() -> Vec<RebootCommand> {
    let input = fs::read_to_string("inputs/22.txt").unwrap();
    input
        .lines()
        .map(|line| RebootCommand::from_str(line))
        .collect::<Result<Vec<RebootCommand>, String>>()
        .unwrap()
}

fn main() {
    part1();
}

fn part1() {
    let commands = get_input();

    println!("part1: {}", 0);
}
