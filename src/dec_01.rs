//! [AOC 2022 Day 1](https://adventofcode.com/2022/day/1)

use std::io;
use std::io::{BufRead, BufReader, Lines, Read};
use std::str::FromStr;

use crate::utils::io_error;

/// Iterates a file of elf calories and sums up total calories for each elf
struct ElfCalories<R> {
    lines: Lines<BufReader<R>>,
}

impl<R> ElfCalories<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: BufReader::new(input).lines()
        }
    }
}

impl<R> Iterator for ElfCalories<R>
    where R: Read
{
    type Item = io::Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total_calories = None;

        loop {
            if let Some(next) = self.lines.next() {
                match next {
                    // only parse and compute lines with values
                    Ok(value) if !value.trim().is_empty() =>
                    // parse string value
                        match usize::from_str(&value) {
                            // if first value initialize total calories
                            Ok(value) if total_calories.is_none() =>
                                total_calories = Some(value),
                            // if existing value add to total calories
                            Ok(value) => {
                                total_calories = total_calories.map(|v| v + value);
                            }
                            // bubble usize paring errors
                            Err(err) => return Some(Err(io_error(&format!("{value}: {err}"))))
                        },
                    // if empty line indicator and we have a value return
                    Ok(_) if total_calories.is_some() =>
                        return total_calories.map(Ok),
                    // continue until value is found
                    Ok(_) => {}
                    // bubble io errors
                    Err(err) =>
                        return Some(Err(err)),
                }
            } else {
                // no more lines return last result
                return total_calories.map(Ok);
            }
        }
    }
}

// find the calories of the elf carrying the most
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let elf_calorie_counter = ElfCalories::new(input);
    let max_elf_calories = elf_calorie_counter
        .into_iter()
        .reduce(|acc, nxt| Ok(acc?.max(nxt?)));

    match max_elf_calories {
        None => Ok(Box::new(0)),
        Some(Ok(value)) => Ok(Box::new(value)),
        Some(Err(err)) => Err(err)
    }
}

// find the total calories of the top three elves carrying the most
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let elf_calorie_counter = ElfCalories::new(input);
    let mut elf_calories = elf_calorie_counter
        .into_iter()
        .collect::<io::Result<Vec<_>>>()?;

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));

    Ok(Box::new(elf_calories.into_iter().take(3).sum::<usize>()))
}

#[cfg(test)]
mod test {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn puzzle_one() {
        let expected = "24000";
        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "45000";
        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}