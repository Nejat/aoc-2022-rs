//! [AOC 2022 Day 2](https://adventofcode.com/2022/day/2)

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn puzzle_one() {
        let expected = "15";
        let actual = crate::dec_02_one::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "12";
        let actual = crate::dec_02_two::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}