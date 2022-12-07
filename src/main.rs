#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

#![allow(clippy::items_after_statements)] // code organization is ok

//! AOC 2022 Oxidized 🦀

use std::fs::{File, remove_file};
use std::io;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

mod dec_01;
mod dec_02;
mod dec_02_one;
mod dec_02_two;
mod dec_03;
mod dec_03_one;
mod dec_03_two;
mod dec_04;
mod dec_05;
mod dec_06;
mod utils;

// a collection of puzzles
type Puzzles<'a> = Vec<(&'a str, &'a str, Box<dyn Fn(File) -> io::Result<Box<dyn ToString>>>)>;

const PUZZLE_INPUT_ROOT: &str = "puzzle_input";

fn main() -> io::Result<()> {
    let mut output = get_buffered_writer("aoc-2022-rs-results.txt")?;

    let puzzles: Puzzles = vec![
        ("2022-12-01 puzzle one", "2022-12-01.txt", Box::new(dec_01::puzzle_one)),
        ("2022-12-01 puzzle two", "2022-12-01.txt", Box::new(dec_01::puzzle_two)),
        ("2022-12-02 puzzle one", "2022-12-02.txt", Box::new(dec_02_one::puzzle_one)),
        ("2022-12-02 puzzle two", "2022-12-02.txt", Box::new(dec_02_two::puzzle_two)),
        ("2022-12-03 puzzle one", "2022-12-03.txt", Box::new(dec_03_one::puzzle_one)),
        ("2022-12-03 puzzle two", "2022-12-03.txt", Box::new(dec_03_two::puzzle_two)),
        ("2022-12-04 puzzle one", "2022-12-04.txt", Box::new(dec_04::puzzle_one)),
        ("2022-12-04 puzzle two", "2022-12-04.txt", Box::new(dec_04::puzzle_two)),
        ("2022-12-05 puzzle one", "2022-12-05.txt", Box::new(dec_05::puzzle_one)),
        ("2022-12-05 puzzle two", "2022-12-05.txt", Box::new(dec_05::puzzle_two)),
        ("2022-12-06 puzzle one", "2022-12-06.txt", Box::new(dec_06::puzzle_one)),
        ("2022-12-06 puzzle two", "2022-12-06.txt", Box::new(dec_06::puzzle_two)),
    ];

    for (label, input_file, puzzle) in puzzles {
        let input_file = get_input_file(input_file)?;

        output.write_fmt(format_args!("{}: {}\n", label, puzzle(input_file)?.to_string()))?;
    }

    Ok(())
}

fn get_buffered_writer<P: AsRef<Path>>(output_path: P) -> io::Result<BufWriter<File>> {
    let output_path = output_path.as_ref();

    if output_path.exists() {
        remove_file(output_path)?;
    }

    let output_file = File::create(output_path)?;

    Ok(BufWriter::new(output_file))
}

fn get_input_file(input_file: &str) -> io::Result<File> {
    let mut input_path = PathBuf::from(PUZZLE_INPUT_ROOT);

    input_path.push(input_file);

    File::open(input_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_correct_answers_for_refactoring() {
        let actual = include_str!("../aoc-2022-rs-results.txt");
        let expected = include_str!("../aoc-2022-rs-expected-results.txt");

        assert_eq!(
            actual.split('\n').count(),
            expected.split('\n').count(),
            "Tests count mismatched!"
        );

        actual.lines()
            .zip(expected.lines())
            .for_each(|(actual, expected)| {
                let (actual_test, actual_result) = actual.split_once(':').unwrap();
                let (expected_test, expected_result) = expected.split_once(':').unwrap();

                assert_eq!(actual_test.trim(), expected_test.trim(), "Test Mismatch");
                assert_eq!(actual_result.trim(), expected_result.trim(), "{} failed", actual_test);
            });
    }
}