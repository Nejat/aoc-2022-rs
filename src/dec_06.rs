//! [AOC 2022 Day 6](https://adventofcode.com/2022/day/6)

use std::io;
use std::io::{BufReader, Read};

use crate::utils::io_error;

// find pack start signal
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const PACKET_START_MARKER_SIZE: usize = 4;

    let data_stream = read_data_stream(input)?;

    Ok(find_marker_start(&data_stream, PACKET_START_MARKER_SIZE)
        .ok_or_else(|| io_error("no pack start signal found"))
        .map(Box::new)?)
}

// find start of message signal
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const MESSAGE_START_MARKER_SIZE: usize = 14;

    let data_stream = read_data_stream(input)?;

    Ok(find_marker_start(&data_stream, MESSAGE_START_MARKER_SIZE)
        .ok_or_else(|| io_error("no message start signal found"))
        .map(Box::new)?)
}

fn find_marker_start(data_stream: &str, marker_length: usize) -> Option<usize> {
    // start of marker
    let mut from = 0;
    // window of marker in data stream
    let mut marker = &data_stream[from..1];

    // iterate data stream one char at a time
    for (idx, next) in data_stream.chars().skip(1).enumerate() {
        // check if next char in stream exists marker
        if let Some(found) = marker.find(next) {
            // if found move start of marker window past duplicate value
            from += found + 1;
        }

        // current window of marker in data stream
        marker = &data_stream[from..=(idx + 1)];

        // check if the length matches marker size we need
        if marker.len() == marker_length {
            return Some(from + marker_length);
        }
    }

    None
}

fn read_data_stream<R>(input: R) -> io::Result<String>
    where R: Read
{
    let mut data_stream = String::new();

    BufReader::new(input).read_to_string(&mut data_stream)?;

    Ok(data_stream)
}

#[cfg(test)]
mod tests {
    const NUM_TEST_CASES: usize = 5;

    const TEST_CASES: [&str; NUM_TEST_CASES] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn puzzle_one() {
        const EXPECTED: [&str; NUM_TEST_CASES] = ["7", "5", "6", "10", "11"];

        for (test_case, expected) in TEST_CASES.iter().zip(&EXPECTED) {
            let actual = super::puzzle_one(test_case.as_bytes()).unwrap().to_string();

            assert_eq!(&actual, *expected);
        }
    }

    #[test]
    fn puzzle_two() {
        const EXPECTED: [&str; NUM_TEST_CASES] = ["19", "23", "23", "29", "26"];

        for (test_case, expected) in TEST_CASES.iter().zip(&EXPECTED) {
            let actual = super::puzzle_two(test_case.as_bytes()).unwrap().to_string();

            assert_eq!(&actual, *expected);
        }
    }
}