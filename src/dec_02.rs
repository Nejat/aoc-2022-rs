//! [AOC 2022 Day 2](https://adventofcode.com/2022/day/2)

#[cfg(test)]
mod tests {
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn puzzle_one() {
        let actual = crate::dec_02_one::puzzle_one(INPUT.as_bytes()).unwrap().to_string();
        let expected = "15";

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let actual = crate::dec_02_two::puzzle_two(INPUT.as_bytes()).unwrap().to_string();
        let expected = "12";

        assert_eq!(actual, expected);
    }
}