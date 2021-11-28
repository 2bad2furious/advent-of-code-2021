extern crate aoc;

mod aoc2020;

#[cfg(test)]
mod tests {
    use aoc::aoc2020::day1::part1;
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
