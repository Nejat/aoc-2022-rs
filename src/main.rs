#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

#![allow(clippy::items_after_statements)] // code organization is ok
#![allow(clippy::upper_case_acronyms)]

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
mod dec_07;
mod dec_08;
mod dec_09;
mod dec_10;
mod dec_11;
mod utils;

// a collection of puzzles
type Puzzles<'a> = Vec<(&'a str, &'a str, Box<dyn Fn(File) -> io::Result<Box<dyn ToString>>>)>;

#[cfg(test)]
const EXPECTED_PUZZLE_SOLUTION: &str = "expected puzzle to provide a solution";

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
        ("2022-12-07 puzzle one", "2022-12-07.txt", Box::new(dec_07::puzzle_one)),
        ("2022-12-07 puzzle two", "2022-12-07.txt", Box::new(dec_07::puzzle_two)),
        ("2022-12-08 puzzle one", "2022-12-08.txt", Box::new(dec_08::puzzle_one)),
        ("2022-12-08 puzzle two", "2022-12-08.txt", Box::new(dec_08::puzzle_two)),
        ("2022-12-09 puzzle one", "2022-12-09.txt", Box::new(dec_09::puzzle_one)),
        ("2022-12-09 puzzle two", "2022-12-09.txt", Box::new(dec_09::puzzle_two)),
        ("2022-12-10 puzzle one", "2022-12-10.txt", Box::new(dec_10::puzzle_one)),
        ("2022-12-10 puzzle two", "2022-12-10.txt", Box::new(dec_10::puzzle_two)),
        ("2022-12-11 puzzle one", "2022-12-11.txt", Box::new(dec_11::puzzle_one)),
        ("2022-12-11 puzzle two", "2022-12-11.txt", Box::new(dec_11::puzzle_two)),
    ];

    for (label, input_file, puzzle) in puzzles {
        let input_file = get_input_file(input_file)?;

        output.write_fmt(format_args!("{label}: {}\n", puzzle(input_file)?.to_string()))?;
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
        const TEST_SEPARATOR: char = '\n';
        const RESULT_SEPARATOR: char = ':';

        let actual = include_str!("../aoc-2022-rs-results.txt").trim();
        let expected = include_str!("../aoc-2022-rs-expected-results.txt").trim();

        assert_eq!(
            actual.split(TEST_SEPARATOR).count(),
            expected.split(TEST_SEPARATOR).count(),
            "Tests counts mismatched!"
        );

        actual.lines()
            .zip(expected.lines())
            .for_each(|(actual, expected)| {
                let (actual_test, actual_result) = actual
                    .split_once(RESULT_SEPARATOR)
                    .expect("actual test to be formatted expectedly");

                let (expected_test, expected_result) = expected
                    .split_once(RESULT_SEPARATOR)
                    .expect("expected test to be formatted expectedly");

                assert_eq!(
                    actual_test.trim(),
                    expected_test.trim(),
                    "test mismatch: '{}' != '{}'", actual_test.trim(), expected_test.trim()
                );

                assert_eq!(
                    actual_result.trim(),
                    expected_result.trim(),
                    "{actual_test} failed"
                );
            });
    }
}