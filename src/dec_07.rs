//! [AOC 2022 Day 7](https://adventofcode.com/2022/day/7)

use std::{io, usize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Read;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::{CleansedLines, io_error, sum_everything};

/// Interpreted CLI session
#[allow(non_camel_case_types)]
enum CLI {
    cd_back(usize),
    cd_folder(String),
    cd_root,
    dir(String),
    file(String, usize),
    ls,
}

impl FromStr for CLI {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let failed = |msg| format!("parsing '{input}' failed; {msg}");

        let mut parts = input.split_whitespace();
        let next = parts.next().ok_or_else(|| failed("incomplete entry"))?;

        Ok(match next {
            "$" => {
                let next = parts.next().ok_or_else(|| failed("incomplete command entry"))?;

                match next {
                    "ls" => Self::ls,
                    "cd" => {
                        let next = parts.next().ok_or_else(|| failed("incomplete cd entry"))?;

                        if next == "/" {
                            Self::cd_root
                        } else if next.starts_with("..") {
                            Self::cd_back(next.split('/').count())
                        } else {
                            Self::cd_folder(next.to_string())
                        }
                    }
                    _ => return Err(format!("unexpected command '{next}'"))
                }
            }
            "dir" => {
                Self::dir(parts.next().ok_or_else(|| failed("expected folder name"))?.to_string())
            }
            _ => {
                Self::file(
                    parts.next().ok_or_else(|| failed("expected file name"))?.to_string(),
                    next.parse().map_err(|err| format!("'{input}'; {err}"))?,
                )
            }
        })
    }
}

/// Interprets a CLI session log
struct CLIInterpreter<R> {
    lines: CleansedLines<R>,
}

impl<R> CLIInterpreter<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl<R> Iterator for CLIInterpreter<R>
    where R: Read
{
    type Item = io::Result<CLI>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.lines.next()?.ok()?;

        Some(
            next.parse::<CLI>()
                .map_err(
                    |err| io_error(&format!("couldn't interpret '{next}' - {err}"))
                )
        )
    }
}

// find all folders at most 100,000 bytes in size
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const THRESHOLD: RangeInclusive<usize> = 0..=100_000;

    let matching_folders = rummage_drive(input)?
        .filter_map(move |total| {
            if THRESHOLD.contains(&total) {
                Some(Ok::<usize, io::Error>(total))
            } else {
                None
            }
        });

    Ok(sum_everything(matching_folders).map(Box::new)?)
}

// find one folder to clear to free a minimum of 30,000,000 bytes
// from a drive capacity of 70,000,000 bytes
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const TOTAL_DRIVE_SIZE: usize = 70_000_000;
    const TARGET_FREE: usize = 30_000_000;

    let contents = rummage_drive(input)?.collect::<Vec<_>>();
    let total_used = contents.iter().max().copied().unwrap_or_default();
    let total_free = TOTAL_DRIVE_SIZE - total_used;
    let need_to_free = TARGET_FREE - total_free;

    let solution = contents.into_iter()
        .filter(|size| *size >= need_to_free)
        .min()
        .unwrap_or_default();

    Ok(Box::new(solution))
}

/// traverse a cli session log to collect folder sizes in bytes
fn rummage_drive<R>(input: R) -> io::Result<impl Iterator<Item=usize>>
    where R: Read,
{
    let mut current = PathBuf::default();
    let mut all_folders: HashMap<OsString, usize> = HashMap::default();
    let cli = CLIInterpreter::new(input);

    for command in cli {
        let command = command?;

        match &command {
            CLI::cd_back(levels) =>
                (0..*levels).into_iter().for_each(|_| {
                    current.pop();
                }),
            CLI::cd_folder(folder) => {
                current.push(folder);

                let path = current.clone().into_os_string();

                all_folders.entry(path).or_insert_with(usize::default);
            }
            CLI::cd_root =>
                current.clear(),
            CLI::file(_file, size) => {
                let mut current = current.clone();

                loop {
                    let path = current.as_os_str();
                    let entry = all_folders.entry(path.to_os_string()).or_insert_with(usize::default);

                    *entry += *size;

                    if !current.pop() {
                        break;
                    }
                }
            }
            CLI::dir(folder) => {
                let mut path = current.clone();

                path.push(folder);

                let path = path.clone().into_os_string();

                all_folders.entry(path).or_insert_with(usize::default);
            }
            CLI::ls => {}
        }
    }

    Ok(all_folders.into_iter().map(|(_key, value)| value))
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn puzzle_one() {
        let expected = "95437";
        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "24933642";
        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}

