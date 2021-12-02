#[cfg(test)]
mod tests {
    use aoc::aoc2021::day2::{Instruction, part1, part2};
    use aoc::aoc2021::day2::Command::{DOWN, FORWARD, UP};

    const INSTRUCTIONS: [Instruction; 6] = [
        Instruction { command: FORWARD, by: 5 },
        Instruction { command: DOWN, by: 5 },
        Instruction { command: FORWARD, by: 8 },
        Instruction { command: UP, by: 3 },
        Instruction { command: DOWN, by: 8 },
        Instruction { command: FORWARD, by: 2 }
    ];

    #[test]
    pub fn test_part1_works() {
        assert_eq!(150, part1(&INSTRUCTIONS))
    }

    #[test]
    pub fn test_part2_works() {
        assert_eq!(900, part2(&INSTRUCTIONS))
    }
}
