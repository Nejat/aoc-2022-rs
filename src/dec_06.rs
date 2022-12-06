use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

use crate::utils::io_error;

// find pack start signal
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
    let data_stream = read_data_stream(input)?;

    Ok(find_marker_start(&data_stream, 4)
        .ok_or_else(|| io_error("no pack start signal found"))
        .map(Box::new)?)
}

// fin message start signal
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    let data_stream = read_data_stream(input)?;

    Ok(find_marker_start(&data_stream, 14)
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

fn read_data_stream(input: File) -> io::Result<String> {
    let mut data_stream = String::new();

    BufReader::new(input).read_to_string(&mut data_stream)?;

    Ok(data_stream)
}
