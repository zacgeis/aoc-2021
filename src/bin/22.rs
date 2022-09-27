use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Op {
    On,
    Off,
}

#[derive(Debug)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl FromStr for Cuboid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let mut coords = vec![];
        while let Some(pos) = parts.next() {
            let mut range_parts = pos[2..].split("..");
            let start = range_parts.next().unwrap();
            let end = range_parts.next().unwrap();
            coords.push((start.parse().unwrap(), end.parse().unwrap()));
        }
        assert_eq!(coords.len(), 3);
        Ok(Cuboid {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

#[derive(Debug)]
struct RebootCommand {
    op: Op,
    cuboid: Cuboid,
}

impl FromStr for RebootCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let op = match parts.next().unwrap() {
            "on" => Op::On,
            "off" => Op::Off,
            _ => panic!("unexpected op"),
        };
        let cuboid = Cuboid::from_str(parts.next().unwrap())?;
        Ok(RebootCommand { op, cuboid })
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
    let filtered = commands.iter().filter(|command| {
        command.cuboid.x.0 >= -50
            && command.cuboid.x.1 <= 50
            && command.cuboid.y.0 >= -50
            && command.cuboid.y.1 <= 50
            && command.cuboid.z.0 >= -50
            && command.cuboid.z.1 <= 50
    });
    println!("debug: {:?}", &commands[0]);
    println!("len: {}", filtered.count());

    println!("part1: {}", 0);
}
