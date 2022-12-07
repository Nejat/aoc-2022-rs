//! [AOC 2022 Day 4](https://adventofcode.com/2022/day/4)

use std::cmp::{max, min};
use std::io;
use std::io::{Error, Read};
use std::ops::RangeInclusive;

use crate::utils::{CleansedLines, io_error, sum_everything};

/// Iterates a file of elf chore assignment pairs
struct ElfChoreAssignmentsPairs<R> {
    lines: CleansedLines<R>,
}

impl<R> ElfChoreAssignmentsPairs<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

type PairResult = io::Result<(RangeInclusive<usize>, RangeInclusive<usize>)>;

impl<R> Iterator for ElfChoreAssignmentsPairs<R>
    where R: Read
{
    type Item = PairResult;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.lines.next()?.ok()?;

        return Some(parse_pair(&pair));


        // splits a pair of ranges
        fn parse_pair(pair: &str) -> PairResult {
            let (section_a, section_b) = split_clean(
                pair, ',',
                || format!("'{pair}' is invalid"), )?;

            let section_a = parse_section(section_a, "first", pair)?;
            let section_b = parse_section(section_b, "Second", pair)?;

            Ok((section_a, section_b))
        }

        // parse a section's range
        fn parse_section(section: &str, name: &str, pair: &str) -> io::Result<RangeInclusive<usize>> {
            let (start, end) = split_clean(
                section, '-',
                || format!("{name} section '{section}' in pair '{pair}' is invalid"),
            )?;

            let parse = |val: &str, which: &str| val.parse::<usize>()
                .map_err(|err| io_error(&format!("{which} value for {name} section '{section}' in pair '{pair}' is invalid; {err}")));

            let start = parse(start, "start")?;
            let end = parse(end, "start")?;

            Ok(start..=end)
        }

        // clean splits a delimited value
        fn split_clean<Err>(pair: &str, delimiter: char, error: Err) -> Result<(&str, &str), Error>
            where Err: Fn() -> String
        {
            pair.split_once(delimiter)
                .ok_or_else(|| io_error(&error()))
                .map(|(a, b)| (a.trim(), b.trim()))
        }
    }
}

// finds all pairs with contained sections
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let chores = ElfChoreAssignmentsPairs::new(input);

    let contained_chores = chores.into_iter()
        .map(
            |pair| pair
                .map(|(a, b)|
                    u32::from(
                        (a.start() <= b.start() && a.end() >= b.end()) ||
                            (a.start() >= b.start() && a.end() <= b.end())
                    )
                )
        );

    Ok(sum_everything(contained_chores).map(Box::new)?)
}

// finds all pairs with overlapping sections
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let chores = ElfChoreAssignmentsPairs::new(input);

    let overlapping_chores = chores.into_iter()
        .map(
            |pair| pair
                .map(|(a, b)|
                    u32::from(max(a.start(), b.start()) <= min(a.end(), b.end()))
                )
        );

    Ok(sum_everything(overlapping_chores).map(Box::new)?)
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn puzzle_one() {
        let expected = "2";
        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "4";
        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}