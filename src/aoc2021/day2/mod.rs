use std::convert::TryInto;
use std::io::{Error, ErrorKind};
use std::process::exit;
use std::str::FromStr;

use aoc2021::read_asset;

fn get_input() -> Vec<Instruction> {
    return read_asset(2).split("\n")
        .take_while(|l| !l.is_empty())
        .map(|line| -> Instruction {
            let mut split = line.split_whitespace();
            let command: Command = Command::from(split.next().unwrap()).unwrap();
            let by: u32 = FromStr::from_str(split.next().unwrap()).unwrap();
            ;
            Instruction { command, by }
        }).collect();
}

pub fn solve_part1() {
    let input = get_input();
    println!("I have {} instructions", input.len());
    println!("{}", part1(input.as_ref()));
}

pub fn solve_part2(){
    let input = get_input();
    println!("I have {} instructions", input.len());
    println!("{}", part2(input.as_ref()));
}

pub fn part1(instructions: &[Instruction]) -> u64 {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;

    for i in instructions {
        match i.command {
            Command::FORWARD => { distance += i.by }
            Command::DOWN => { depth += i.by }
            Command::UP => { depth -= i.by }
        }
    }

    return (depth as u64) * (distance as u64);
}

pub fn part2(instructions: &[Instruction]) -> u64 {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;
    let mut aim: i32 = 0;

    for i in instructions {
        match i.command {
            Command::FORWARD => {
                distance += i.by;
                depth = ((depth as i32) + (aim * (i.by as i32))) as u32;
            }
            Command::DOWN => { aim += (i.by as i32) }
            Command::UP => { aim -= (i.by as i32) }
        }
    }

    return (depth as u64) * (distance as u64)
}

pub enum Command {
    FORWARD,
    DOWN,
    UP,
}

pub struct Instruction {
    pub command: Command,
    pub by: u32,
}

impl Command {
    fn from(s: &str) -> Result<Command, Error> {
        return match s.to_uppercase().as_str() {
            "FORWARD" => Ok(Command::FORWARD),
            "UP" => Ok(Command::UP),
            "DOWN" => Ok(Command::DOWN),
            _ => Err(Error::new(ErrorKind::InvalidData, s))
        };
    }
}
