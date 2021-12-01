use super::*;

#[cfg(test)]
mod tests {
    use std::option::Option;
    use std::option::Option::None;
    use aoc::aoc2021::day1::{solve_part1, solve_part2};

    const TEST_NUMBERS: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_solve_part1_works() {
        assert_eq!(7, solve_part1(&TEST_NUMBERS).unwrap());
    }

    #[test]
    fn test_solve_part1_crashes(){
        assert!( solve_part1(&[]).is_none());
    }

    #[test]
    fn test_solve_part2_works(){
        assert_eq!(5, solve_part2(&TEST_NUMBERS).unwrap())
    }

}
