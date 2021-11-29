use super::*;

#[cfg(test)]
mod tests {
    use aoc::aoc2020::day1::{part1, part2};

    const TEST_NUMBERS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_solve_part1_works() {
        assert_eq!(514579, part1(&TEST_NUMBERS).unwrap_or(0));
    }

    #[test]
    fn test_solve_part1_crashes(){
        assert!(part1(&[]).is_err())
    }


    #[test]
    fn test_solve_part2_works(){
        assert_eq!(241861950, part2(&TEST_NUMBERS).unwrap())
    }

    #[test]
    fn test_solve_part2_crashes() {
        assert!(part2(&[]).is_err())
    }

}
