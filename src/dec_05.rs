extern crate nom;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::iter::Peekable;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::IResult;

use crate::utils::{CleansedLines, io_error};

type Crates = VecDeque<String>;
type Labels = Vec<String>;
type Moves = Vec<Move>;
type Stacks = HashMap<String, Crates>;

/// Move instruction
#[derive(Debug)]
struct Move {
    /// Number of crates to move
    crates: usize,
    /// Source crate; move from
    from: String,
    /// Destination crate; move to
    to: String,
}

/// parse input instructions
#[allow(clippy::too_many_lines)]
fn parse_instructions(input: File) -> io::Result<(Labels, Stacks, Moves)> {
    let input = CleansedLines::new(input).peekable();
    let (input, stacks) = parse_stack_crates(input)?;
    let (input, stack_labels) = parse_stack_labels(input)?;

    // check the number of stacks inputted matches the number labels parsed
    if stacks.len() != stack_labels.len() {
        return Err(io_error(&format!(
            "number of stacks {} does not match number of stack labels {}",
            stacks.len(),
            stack_labels.len())
        ));
    }

    let moves = parse_move_instructions(input)?;
    
    // use labels to keep input order
    let stacks = stack_labels
        .clone()
        .into_iter()
        .zip(stacks.into_iter())
        .collect::<HashMap<String, Crates>>();

    return Ok((stack_labels, stacks, moves));

    fn parse_move_instructions<I>(mut input: I) -> io::Result<Moves>
        where I: Iterator<Item=io::Result<String>>,
    {
        let mut moves = Moves::new();

        loop {
            let instruction = match input.next() {
                None if moves.is_empty() =>
                    return Err(io_error("no move instructions found")),
                None =>
                    break,
                Some(instructions) =>
                    instructions.map_err(
                        |err| io_error(&format!("exception reading next move instruction; {err}"))
                    )?,
            };

            let (_, r#move) = parse_move(&instruction)
                .map_err(
                    |err| io_error(&format!("couldn't parsing move instruction '{instruction}'; {err}"))
                )?;

            moves.push(r#move);
        }

        return Ok(moves);

        fn parse_move(instruction: &str) -> IResult<&str, Move> {
            let (next, _) = tag("move")(instruction)?;
            let (next, _) = space1(next)?;
            let (next, crates) = complete::u32(next)?;
            let (next, _) = space1(next)?;
            let (next, _) = tag("from")(next)?;
            let (next, _) = space1(next)?;
            let (next, from) = alphanumeric1(next)?;
            let (next, _) = space1(next)?;
            let (next, _) = tag("to")(next)?;
            let (next, _) = space1(next)?;
            let (next, to) = alphanumeric1(next)?;

            let r#move = Move {
                crates: crates as usize,
                from: from.to_string(),
                to: to.to_string(),
            };

            Ok((next, r#move))
        }
    }

    fn parse_stack_crates<I>(mut input: Peekable<I>) -> io::Result<(Peekable<I>, Vec<Crates>)>
        where
            I: Iterator<Item=io::Result<String>>,
    {
        let mut stacks = Vec::new();

        loop {
            if let Some(Ok(next)) = input.peek() {
                if !next.contains('[') { break; }
            }

            let crates = match input.next() {
                None if stacks.is_empty() =>
                    return Err(io_error("no crate contents found")),
                None =>
                    break,
                Some(crates) =>
                    crates.map_err(
                        |err| io_error(&format!("exception reading crate contents; {err}"))
                    )?,
            };

            let (_, crates) = parse_crates(&crates)
                .map_err(
                    |err| io_error(&format!("couldn't parse crate contents '{crates}'; {err}"))
                )?;

            while stacks.len() < crates.len() {
                stacks.push(Crates::new());
            }

            for (idx, contents) in crates.into_iter().enumerate() {
                if let Some(contents) = contents {
                    stacks[idx].push_back(contents);
                }
            }
        }

        return Ok((input, stacks));

        fn parse_crates(input: &str) -> IResult<&str, Vec<Option<String>>> {
            let mut crates = Vec::new();
            let mut stack = 0;
            let mut remainder = input.trim_end();

            loop {
                let (next, empty) = space0(remainder)?;

                stack += empty.len() >> 2;

                if next.contains('[') {
                    let (next, _) = tag("[")(next)?;
                    let (next, contents) = alphanumeric1(next)?;
                    let (next, _) = tag("]")(next)?;

                    while crates.len() < stack {
                        crates.push(None);
                    }

                    crates.push(Some(contents.to_string()));

                    stack += 1;

                    remainder = next;
                } else {
                    remainder = next;
                    break;
                }
            }

            Ok((remainder, crates))
        }
    }

    fn parse_stack_labels<I>(mut input: I) -> io::Result<(I, Labels)>
        where I: Iterator<Item=io::Result<String>>,
    {
        let labels = input
            .next()
            .ok_or_else(|| io_error("could not find any labels"))?
            .map_err(|err| io_error(&format!("exception reading stack labels; {err}")))?;

        let labels = labels
            .split(' ')
            .filter_map(|v| if v.trim().is_empty() { None } else { Some(v.trim().to_string()) })
            .collect::<Vec<_>>();

        Ok((input, labels))
    }
}

// finds crates at the top of each stack after all of the move instructions;
// moving each crate one at a time
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
    let (labels, mut stacks, instructions) = parse_instructions(input)?;

    for r#move in instructions {
        for _ in 0..r#move.crates {
            let from_crate = stacks
                .get_mut(&r#move.from)
                .ok_or_else(|| io_error(&format!("could not find stack '{}'", r#move.from)))?
                .pop_front()
                .ok_or_else(|| io_error(&format!("expected more crates on stack '{}'", r#move.from)))?;

            let to_crate = stacks
                .get_mut(&r#move.to)
                .ok_or_else(|| io_error(&format!("could not find stack '{}'", r#move.to)))?;

            to_crate.push_front(from_crate);
        }
    }

    Ok(Box::new(top_crate_off_all_stacks(labels, stacks)))
}

// finds crates at the top of each stack after all of the move instructions;
// moving all crates,to be moved, at once preserving stacking order
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    let (labels, mut stacks, instructions) = parse_instructions(input)?;

    for r#move in instructions {
        let mut moved_crates = VecDeque::new();

        let from_crate = stacks
            .get_mut(&r#move.from)
            .ok_or_else(|| io_error(&format!("could not find stack '{}'", r#move.from)))?;

        for _ in 0..r#move.crates {
            let from_crate = from_crate
                .pop_front()
                .ok_or_else(|| io_error(&format!("expected more crates on stack '{}'", r#move.from)))?;

            moved_crates.push_front(from_crate);
        }

        let to_crate = stacks
            .get_mut(&r#move.to)
            .ok_or_else(|| io_error(&format!("could not find stack '{}'", r#move.to)))?;

        for moved_crate in moved_crates {
            to_crate.push_front(moved_crate);
        }
    }

    Ok(Box::new(top_crate_off_all_stacks(labels, stacks)))
}

fn top_crate_off_all_stacks(labels: Labels, mut stacks: Stacks) -> String {
    labels.into_iter()
        .filter_map(|lbl| stacks.get_mut(&lbl)?.pop_front())
        .collect()
}