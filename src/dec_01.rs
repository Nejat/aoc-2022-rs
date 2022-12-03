use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

use crate::utils::io_error;

/// Iterates a file of elf calories and sums up total calories for each elf
struct ElfCalories {
    lines: Lines<BufReader<File>>,
}

impl ElfCalories {
    fn new(input: File) -> Self {
        Self {
            lines: BufReader::new(input).lines()
        }
    }
}

impl Iterator for ElfCalories {
    type Item = io::Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total_calories = None;

        loop {
            match self.lines.next() {
                // no more lines return last result
                None =>
                    return total_calories.map(Ok),
                Some(next_line) =>
                    match next_line {
                        // only parse and compute lines with values
                        Ok(value) if !value.trim().is_empty() =>
                        // parse string value
                            match usize::from_str(&value) {
                                // if first value initialize total calories
                                Ok(value) if total_calories.is_none() =>
                                    total_calories = Some(value),
                                // if existing value add to total calories
                                Ok(value) => {
                                    total_calories.map(|v| v + value);
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
            };
        }
    }
}

// find the calories of the elf carrying the most
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
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
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    let elf_calorie_counter = ElfCalories::new(input);
    let mut elf_calories = elf_calorie_counter
        .into_iter()
        .collect::<io::Result<Vec<_>>>()?;

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));

    Ok(Box::new(elf_calories.into_iter().take(3).sum::<usize>()))
}
