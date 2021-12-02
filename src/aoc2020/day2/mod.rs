use std::convert::TryInto;
use std::ops::{Range, RangeInclusive};

use regex::Regex;

use aoc2020::read_asset;

pub fn get_input() -> Vec<PasswordConfig> {
    let pattern: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    return read_asset(2)
        .lines()
        .map(|s| {
            let captures = pattern.captures(s).unwrap();
            let start: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let end: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let range = RangeInclusive::new(start, end);
            let ch: char = captures.get(3).unwrap().as_str().chars().next().unwrap();
            let word: String = captures.get(4).unwrap().as_str().into();
            return PasswordConfig { range, ch, word };
        })
        .collect();
}

pub fn solve_part1() {
    let input = get_input();
    println!("{}", part1(input.as_ref()))
}

pub fn solve_part2(){
    let input = get_input();
    println!("{}", part2(input.as_ref()))
}

pub fn part1(passwords: &[PasswordConfig]) -> usize {
    return passwords.iter()
        .filter(|p| -> bool {
            p.range.contains(&(p.word.chars().filter(|c| c == &p.ch).count() as u32))
        })
        .count();
}

pub fn part2(passwords: &[PasswordConfig]) -> usize {
    return passwords.iter()
        .filter(|p| -> bool {
            let some_char = Some(p.ch);
            (p.word.chars().nth((p.range.start() - 1)as usize) == some_char) ^ (p.word.chars().nth((p.range.end() - 1) as usize) == some_char)
        })
        .count();
}

pub struct PasswordConfig {
    pub range: RangeInclusive<u32>,
    pub ch: char,
    pub word: String,
}
