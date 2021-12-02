use std::ops::{Range, RangeInclusive};
use aoc2020::read_asset;

/*const RE: regex::Regex = "a";

pub fn get_input()-> &[PasswordConfig<&str>] {
    return read_asset(2)
        .split_whitespace()
        .map(|s| {
            RE.
            PasswordConfig {}
        })
        .collect()
}*/

pub fn solve_part1(){

}

pub fn part1(passwords: &[PasswordConfig<&str>]) -> usize {
    return passwords.iter()
        .filter(|p| -> bool {
            p.range.contains(&(p.word.chars().filter(|c| c == &p.ch).count() as u32))
        })
        .count();
}


pub struct PasswordConfig<T: Sized> {
    pub range: RangeInclusive<u32>,
    pub ch: char,
    pub word: T,
}
