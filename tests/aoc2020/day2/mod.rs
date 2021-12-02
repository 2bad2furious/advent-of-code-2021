#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use aoc::aoc2020::day2::{part1, part2, PasswordConfig};

    fn test_input() -> [PasswordConfig; 3] {
        return [
            PasswordConfig { range: RangeInclusive::new(1, 3), ch: 'a', word: "abcde".to_string() },
            PasswordConfig { range: RangeInclusive::new(1, 3), ch: 'b', word: "cdefg".to_string() },
            PasswordConfig { range: RangeInclusive::new(2, 9), ch: 'c', word: "ccccccccc".to_string() }
        ];
    }

    #[test]
    fn test_part1_works() {
        assert_eq!(2, part1(&test_input()))
    }

    #[test]
    fn test_part2_works() {
        assert_eq!(1, part2(&test_input()))
    }
}
