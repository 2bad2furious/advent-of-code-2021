use super::*;

#[cfg(test)]
mod tests {
    use aoc::aoc2020::day1::part1;

    #[test]
    fn test_solve_works() {
        assert_eq!(514579, part1(&vec![1721, 979, 366, 299, 675, 1456]).unwrap_or(0));
    }

    #[test]
    fn test_solve_crashes(){
        assert!(part1(&vec![]).is_err())
    }
}
