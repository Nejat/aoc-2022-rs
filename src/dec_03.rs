//! [AOC 2022 Day 3](https://adventofcode.com/2022/day/3)

#[cfg(test)]
mod tests {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn puzzle_one() {
        let actual = crate::dec_03_one::puzzle_one(INPUT.as_bytes()).unwrap().to_string();
        let expected = "157";

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let actual = crate::dec_03_two::puzzle_two(INPUT.as_bytes()).unwrap().to_string();
        let expected = "70";

        assert_eq!(actual, expected);
    }
}