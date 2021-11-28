use std::fs::File;
use std::io::{Error, ErrorKind};

use aoc2020::read_asset;

pub fn solve_part1() {
    let mut input: Vec<i32> = Vec::new();
    for x in read_asset(1).split_whitespace() {
        input.push(x.parse().unwrap());
    };
    println!("{}", part1(input.as_ref()).unwrap());
}

pub fn part1(numbers: &Vec<i32>) -> Result<i32, Error> {
    for x in numbers {
        for y in numbers {
            if x + y == 2020 {
                return Ok(x * y);
            }
        }
    }
    return Err(Error::from(ErrorKind::InvalidInput));
}
