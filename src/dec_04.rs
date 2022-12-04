use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::Error;
use std::ops::RangeInclusive;

use crate::utils::{CleansedLines, io_error, sum_everything};

/// Iterates a file of elf chore assignment pairs
struct ElfChoreAssignmentsPairs {
    lines: CleansedLines,
}

impl ElfChoreAssignmentsPairs {
    fn new(input: File) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

type PairResult = io::Result<(RangeInclusive<usize>, RangeInclusive<usize>)>;

impl Iterator for ElfChoreAssignmentsPairs {
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
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
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
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
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