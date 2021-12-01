#[cfg(test)]
mod tests{
    use aoc::aoc2020::day13::part2;

    #[test]
    pub fn test_works(){
        assert_eq!(3417, part2(&[Some(17), None, Some(13), Some(19)]).unwrap());
        assert_eq!(754018, part2(&[Some(67), Some(7), Some(59), Some(61)]).unwrap());
        assert_eq!(779210, part2(&[Some(67), None, Some(7), Some(59), Some(61)]).unwrap());
        assert_eq!(1261476, part2(&[Some(67), Some(7), None, Some(59), Some(61)]).unwrap());
        assert_eq!(1202161486, part2(&[Some(1789), Some(37), Some(47), Some(1889)]).unwrap());
    }
}
