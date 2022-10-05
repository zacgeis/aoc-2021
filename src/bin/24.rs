use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Reg {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl FromStr for Reg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Reg::X),
            "y" => Ok(Reg::Y),
            "z" => Ok(Reg::Z),
            "w" => Ok(Reg::W),
            _ => Err("Invalid reg.".to_string()),
        }
    }
}

#[derive(Debug)]
enum RegOrIm {
    Reg(Reg),
    Im(i64),
}

impl FromStr for RegOrIm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap().is_alphabetic() {
            Ok(RegOrIm::Reg(Reg::from_str(s)?))
        } else {
            Ok(RegOrIm::Im(s.parse().unwrap()))
        }
    }
}

#[derive(Debug)]
enum Inst {
    Inp(Reg),
    Add(Reg, RegOrIm),
    Mul(Reg, RegOrIm),
    Div(Reg, RegOrIm),
    Mod(Reg, RegOrIm),
    Eql(Reg, RegOrIm),
}

impl FromStr for Inst {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let inst = parts.next().unwrap();
        let reg = Reg::from_str(parts.next().unwrap())?;
        match inst {
            "inp" => Ok(Inst::Inp(reg)),
            "add" => {
                let reg_or_im = RegOrIm::from_str(parts.next().unwrap())?;
                Ok(Inst::Add(reg, reg_or_im))
            }
            "mul" => {
                let reg_or_im = RegOrIm::from_str(parts.next().unwrap())?;
                Ok(Inst::Mul(reg, reg_or_im))
            }
            "div" => {
                let reg_or_im = RegOrIm::from_str(parts.next().unwrap())?;
                Ok(Inst::Div(reg, reg_or_im))
            }
            "mod" => {
                let reg_or_im = RegOrIm::from_str(parts.next().unwrap())?;
                Ok(Inst::Mod(reg, reg_or_im))
            }
            "eql" => {
                let reg_or_im = RegOrIm::from_str(parts.next().unwrap())?;
                Ok(Inst::Eql(reg, reg_or_im))
            }
            _ => Err("Invalid inst.".to_string()),
        }
    }
}

struct Alu {
    registers: [i64; 4],
}

impl Alu {
    fn run_program(&mut self, insts: &[Inst]) {
        todo!()
    }

    fn run_inst(&mut self, inst: &Inst) {
        match inst {
            Inst::Inp(r) => (),
            Inst::Add(r, v) => (),
            Inst::Mul(r, v) => (),
            Inst::Div(r, v) => (),
            Inst::Mod(r, v) => (),
            Inst::Eql(r, v) => (),
        }
        todo!()
    }

    fn get_reg(&self, r: Reg) -> i64 {
        self.registers[r as usize]
    }

    fn set_reg(&mut self, r: Reg, v: i64) {
        self.registers[r as usize] = v;
    }
}

fn get_input() -> Vec<Inst> {
    let input = fs::read_to_string("inputs/24.txt").unwrap();
    input
        .lines()
        .map(|line| Inst::from_str(line).unwrap())
        .collect()
}

fn main() {
    part1();
}

fn part1() {
    let insts = get_input();
    println!("part1: {:?}", insts);
}
