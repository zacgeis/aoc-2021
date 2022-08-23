use std::{fs, str::FromStr};

fn main() {
    part1();
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

// TODO: Move below into this.
// impl FromStr for Command {
// }

fn get_input() -> Vec<Command> {
    let input = fs::read_to_string("inputs/02.txt").unwrap();
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        assert!(parts.len() == 2);
        let direction = parts[0];
        let count = parts[1].parse().unwrap();
        match direction {
            "forward" => Command::Forward(count),
            "down" => Command::Down(count),
            "up" => Command::Up(count),
            _ => panic!("Unknown direction: {}", direction)
        }
    }).collect()
}

fn part1() {
    // TODO
    let input = get_input();
    let deb = &input[..3];
    deb.iter().for_each(|command| println!("{:?}", command));
}
