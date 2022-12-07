//! [AOC 2022 Day 3](https://adventofcode.com/2022/day/3)

use std::io;
use std::io::Read;

use crate::dec_03_one::prioritize_rucksack_item;
use crate::utils::{CleansedLines, find_matching, io_error, sum_everything};

/// Iterates a file of elf rucksacks and rummages around for improperly placed items
struct RummageRucksacks<R> {
    lines: CleansedLines<R>,
}

impl<R> RummageRucksacks<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl<R> Iterator for RummageRucksacks<R>
    where R: Read
{
    type Item = io::Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        const NOT_ENOUGH: &str = "Not enough rucksacks to make a required group";

        let group_a = match self.lines.next() {
            Some(Ok(group)) => group,
            Some(Err(err)) => return Some(Err(err)),
            None => return None,
        };
        let group_b = match self.lines.next() {
            Some(Ok(group)) => group,
            Some(Err(err)) => return Some(Err(err)),
            None => return Some(Err(io_error(NOT_ENOUGH))),
        };
        let group_c = match self.lines.next() {
            Some(Ok(group)) => group,
            Some(Err(err)) => return Some(Err(err)),
            None => return Some(Err(io_error(NOT_ENOUGH))),
        };

        let group_a = group_a.chars();
        let group_b = group_b.chars();
        let group_c = group_c.chars();

        // group rucksacks into groups of three
        let elf_groups = [group_a, group_b, group_c];

        let match_results = find_matching(
            &elf_groups,
            |itm| itm.is_alphabetic()
                .then_some(())
                .ok_or_else(|| io_error("Not all items in rucksacks are valid items")),
            io_error,
        );

        match_results.map_or(None, |found| Some(Ok(prioritize_rucksack_item(found))))
    }
}

// solve some puzzle one
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    Ok(sum_everything(RummageRucksacks::new(input)).map(Box::new)?)
}
