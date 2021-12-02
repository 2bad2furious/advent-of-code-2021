#[cfg(test)]
mod tests {
    use std::ops::{RangeInclusive};

    use aoc::aoc2020::day2::{part1, PasswordConfig};

    const INPUT: [PasswordConfig<&str>; 3] = [
        PasswordConfig { range: RangeInclusive::new(1, 3), ch: 'a', word: "abcde" },
        PasswordConfig { range: RangeInclusive::new(1,3), ch: 'b', word: "cdefg" },
        PasswordConfig { range: RangeInclusive::new(2, 9), ch: 'c', word: "ccccccccc" }
    ];

    #[test]
    fn test_part1_works() {
        assert_eq!(2, part1(&INPUT))
    }
}
