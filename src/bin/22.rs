use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Mapping {
    x_to_compressed: HashMap<i64, i64>,
    compressed_to_x: HashMap<i64, i64>,
    y_to_compressed: HashMap<i64, i64>,
    compressed_to_y: HashMap<i64, i64>,
    z_to_compressed: HashMap<i64, i64>,
    compressed_to_z: HashMap<i64, i64>,
}

impl Mapping {
    fn new(x_values: &[i64], y_values: &[i64], z_values: &[i64]) -> Self {
        let mut x_values = x_values.to_vec();
        x_values.sort();
        x_values.dedup();

        let mut x_to_compressed = HashMap::new();
        let mut compressed_to_x = HashMap::new();
        for (i, v) in x_values.iter().enumerate() {
            x_to_compressed.insert(*v, i as i64);
            compressed_to_x.insert(i as i64, *v);
        }

        let mut y_values = y_values.to_vec();
        y_values.sort();
        y_values.dedup();

        let mut y_to_compressed = HashMap::new();
        let mut compressed_to_y = HashMap::new();
        for (i, v) in y_values.iter().enumerate() {
            y_to_compressed.insert(*v, i as i64);
            compressed_to_y.insert(i as i64, *v);
        }

        let mut z_values = z_values.to_vec();
        z_values.sort();
        z_values.dedup();

        let mut z_to_compressed = HashMap::new();
        let mut compressed_to_z = HashMap::new();
        for (i, v) in z_values.iter().enumerate() {
            z_to_compressed.insert(*v, i as i64);
            compressed_to_z.insert(i as i64, *v);
        }

        Mapping {
            x_to_compressed,
            compressed_to_x,
            y_to_compressed,
            compressed_to_y,
            z_to_compressed,
            compressed_to_z,
        }
    }

    fn compress(&self, cuboid: &Cuboid) -> Cuboid {
        let x = (
            self.x_to_compressed[&cuboid.x.0],
            self.x_to_compressed[&(cuboid.x.1 + 1)],
        );
        let y = (
            self.y_to_compressed[&cuboid.y.0],
            self.y_to_compressed[&(cuboid.y.1 + 1)],
        );
        let z = (
            self.z_to_compressed[&cuboid.z.0],
            self.z_to_compressed[&(cuboid.z.1 + 1)],
        );
        Cuboid { x, y, z }
    }

    fn uncompressed_point_size(&self, point: &(i64, i64, i64)) -> i64 {
        let (x, y, z) = point;
        let x_start = self.compressed_to_x.get(&x).unwrap();
        let x_diff = match self.compressed_to_x.get(&(x + 1)) {
            Some(x_end) => (x_end - x_start).abs(),
            None => 1,
        };

        let y_start = self.compressed_to_y.get(&y).unwrap();
        let y_diff = match self.compressed_to_y.get(&(y + 1)) {
            Some(y_end) => (y_end - y_start).abs(),
            None => 1,
        };

        let z_start = self.compressed_to_z.get(&z).unwrap();
        let z_diff = match self.compressed_to_z.get(&(z + 1)) {
            Some(z_end) => (z_end  - z_start).abs(),
            None => 1,
        };

        x_diff * y_diff * z_diff
    }
}

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
    part2();
}

fn solve(mut commands: Vec<RebootCommand>) -> i64 {
    let mut x_range = vec![];
    let mut y_range = vec![];
    let mut z_range = vec![];
    for command in commands.iter() {
        x_range.push(command.cuboid.x.0);
        x_range.push(command.cuboid.x.1 + 1);

        y_range.push(command.cuboid.y.0);
        y_range.push(command.cuboid.y.1 + 1);

        z_range.push(command.cuboid.z.0);
        z_range.push(command.cuboid.z.1 + 1);
    }
    let mapping = Mapping::new(&x_range, &y_range, &z_range);

    for command in &mut commands {
        command.cuboid = mapping.compress(&command.cuboid);
    }

    let mut world = HashSet::new();
    for command in &commands {
        for x in command.cuboid.x.0..command.cuboid.x.1 {
            for y in command.cuboid.y.0..command.cuboid.y.1 {
                for z in command.cuboid.z.0..command.cuboid.z.1 {
                    match command.op {
                        Op::On => {
                            world.insert((x, y, z));
                        }
                        Op::Off => {
                            world.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for point in &world {
        result += mapping.uncompressed_point_size(point);
    }
    result
}

fn part1() {
    let commands = get_input();
    let commands = commands
        .into_iter()
        .filter(|command| {
            command.cuboid.x.0 >= -50
                && command.cuboid.x.1 <= 50
                && command.cuboid.y.0 >= -50
                && command.cuboid.y.1 <= 50
                && command.cuboid.z.0 >= -50
                && command.cuboid.z.1 <= 50
        })
        .collect::<Vec<RebootCommand>>();

    println!("part1: {}", solve(commands));
}

fn part2() {
    let commands = get_input();

    println!("part2: {}", solve(commands));
}
