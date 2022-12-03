use std::fs::File;
use std::io;

use crate::utils::{CleansedLines, find_matching, io_error, sum_everything};

/// Iterates a file of elf rucksacks and rummages around for improperly placed items
struct RummageRucksack {
    lines: CleansedLines,
}

impl RummageRucksack {
    fn new(input: File) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl Iterator for RummageRucksack {
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
            || io_error("No matches found in rucksack compartments"),
        );

        match_results.map_or(None, |found| Some(Ok(prioritize_rucksack_item(found))))
    }
}

/// find the miss items in compartments of a rucksack
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
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