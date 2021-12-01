use std::option::Option;
use std::option::Option::Some;
use aoc2021::read_asset;

pub fn get_input() -> Vec<u32> {
    let input: Vec<u32> = read_asset(1)
        .split_whitespace().into_iter()
        .map(|s| -> u32 { s.to_string().parse().unwrap() })
        .collect();
    println!("I have {} numbers", input.len());
    return input;
}

pub fn part1() {
    println!("{}", solve_part1(get_input().as_ref()).unwrap())
}

pub fn solve_part1(depths: &[u32]) -> Option<u32> {
    let mut last: &u32 = depths.first()?;
    let mut count: u32 = 0;
    for depth in depths {
        if depth > last { count = count + 1 };
        last = depth
    }
    return Some(count);
}

pub fn part2(){
    println!("{}", solve_part2(get_input().as_ref()).unwrap())
}

pub fn solve_part2(depths: &[u32]) -> Option<u32>{
    let a = depths.array_windows::<3>();
    let r: [u32; 2] = a.fold([0, 0], |last_with_count, next | {
        let sum: u32 = next.iter().sum::<u32>();
        let increase = if last_with_count[0] != 0 && sum > last_with_count[0] {1} else {0};
        return [sum, last_with_count[1] + increase]
    });
    return Some(r[1])
}
