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
mod dec_02_one;
mod dec_02_two;

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
