use std::fs::File;
use std::io::{Error, ErrorKind};

use aoc2020::read_asset;

const YEAR: u32 = 2020;

pub fn get_input() -> Vec<u32> {
    let input: Vec<u32> = read_asset(1)
        .split_whitespace().into_iter()
        .map(|s| -> u32 { s.to_string().parse().unwrap() })
        .collect();
    println!("I have {} numbers", input.len());
    return input;
}

pub fn solve_part1() {
    let input = get_input();
    println!("{}", part1(input.as_ref()).unwrap());
}

pub fn solve_part2() {
    let input = get_input();
    println!("{}", part2(input.as_ref()).unwrap());
}


pub fn part1(numbers: &[u32]) -> Result<u32, Error> {
    for x in numbers {
        if x > &YEAR { continue; }
        for y in numbers {
            if x + y == YEAR {
                return Ok(x * y);
            }
        }
    }
    return Err(Error::from(ErrorKind::InvalidInput));
}

pub fn part2(numbers: &[u32]) -> Result<u32, Error> {
    for x in numbers {
        if x > &YEAR { continue; }
        for y in numbers {
            let xy_sum = x + y;
            if xy_sum > YEAR { continue; }
            for z in numbers {
                if xy_sum + z == YEAR { return Ok(x * y * z)}
            }
        }
    }
    return Err(Error::from(ErrorKind::InvalidInput));
}
