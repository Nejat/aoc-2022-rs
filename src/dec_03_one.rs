//! [AOC 2022 Day 3](https://adventofcode.com/2022/day/3)

use std::io;
use std::io::Read;

use crate::utils::{CleansedLines, find_matching, io_error, sum_everything};

/// Iterates a file of elf rucksacks and rummages around for improperly placed items
struct RummageRucksack<R> {
    lines: CleansedLines<R>,
}

impl<R> RummageRucksack<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl<R> Iterator for RummageRucksack<R>
    where R: Read
{
    type Item = io::Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let rucksack = self.lines.next()?.ok()?;

        if !rucksack.is_ascii() {
            return Some(Err(io_error(&format!("Not all items in rucksack '{rucksack}' are valid items"))));
        }

        let items = rucksack.len();
        let compartment_items = items >> 1;

        // rucksack compartments are supposed to be the same size
        if items != compartment_items << 1 {
            return Some(Err(io_error(&format!("'{rucksack}' does not have the same number of items in each of two compartments"))));
        }

        // split rucksack into its compartments
        let compartment_a = rucksack[0..compartment_items].chars();
        let compartment_b = rucksack[compartment_items..].chars();

        // find first matching item in each rucksack compartment
        let match_results = find_matching(
            &[compartment_a, compartment_b],
            |itm| itm.is_alphabetic()
                .then_some(())
                .ok_or_else(|| io_error("Not all items in rucksacks are valid items")),
            io_error,
        );

        match_results.map_or(None, |found| Some(Ok(prioritize_rucksack_item(found))))
    }
}

/// find the miss items in compartments of a rucksack
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    Ok(sum_everything(RummageRucksack::new(input)).map(Box::new)?)
}

/// convert rucksack item into it priority
pub const fn prioritize_rucksack_item(found: char) -> usize {
    if found.is_ascii_uppercase() {
        found as usize - 38
    } else {
        found as usize - 96
    }
}