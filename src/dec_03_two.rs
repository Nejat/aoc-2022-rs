use std::fs::File;
use std::io;

use crate::dec_03_one::prioritize_rucksack_item;
use crate::utils::{CleansedLines, find_matching, io_error, sum_everything};

/// Iterates a file of elf rucksacks and rummages around for improperly placed items
struct RummageRucksacks {
    lines: CleansedLines,
}

impl RummageRucksacks {
    fn new(input: File) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl Iterator for RummageRucksacks {
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
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    Ok(sum_everything(RummageRucksacks::new(input)).map(Box::new)?)
}
