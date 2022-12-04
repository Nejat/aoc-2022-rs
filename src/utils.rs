use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Lines};
use std::ops::{Add, ControlFlow};

/// Iterates a file line by line skipping empty lines and honoring io errors
pub struct CleansedLines {
    lines: Lines<BufReader<File>>,
}

impl CleansedLines {
    pub fn new(input: File) -> Self {
        Self {
            lines: BufReader::new(input).lines(),
        }
    }
}

impl Iterator for CleansedLines {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines.next()?.ok()?;

        // read file line by line skipping empty lines, honoring io errors
        while line.trim().is_empty() {
            line = self.lines.next()?.ok()?;
        }

        Some(Ok(line))
    }
}

/// find the first item that is present in all input iterators
pub fn find_matching<I, Iter, IntoIter, P, ER, E, const N: usize>(
    input: &[IntoIter; N],
    mut predicate: P,
    build_error: ER,
) -> Result<Iter::Item, E>
    where I: Eq + Copy,
          Iter: Iterator<Item=I>,
          IntoIter: IntoIterator<Item=I, IntoIter=Iter> + Clone,
          E: Error,
          P: FnMut(I) -> Result<(), E>,
          ER: Fn(&str) -> E,
{
    if input.len() < 2 { return Err(build_error("Provide more than one set to find matching items")); }

    let mut found_match = None;
    let items = input[0].clone().into_iter();

    for item in items {
        predicate(item)?;

        let matches = input
            .iter().skip(1)
            .filter_map(|items| {
                items.clone().into_iter()
                    .find_map(|itm| match predicate(itm) {
                        // validate item failed
                        Err(err) => Some(Err(err)),
                        // validate item passed, item matches
                        Ok(_) if itm == item => Some(Ok(itm)),
                        // validate item passed, item does not match
                        Ok(_) => None,
                    })
            })
            .collect::<Result<Vec<_>, E>>()?;

        if matches.len() == input.len() - 1 {
            found_match = Some(item);
            break;
        }
    }

    found_match.ok_or_else(|| build_error("No matches found"))
}

/// create an `io::Error`
#[inline]
pub fn io_error(error: &str) -> io::Error {
    io::Error::new(ErrorKind::Other, error)
}

// sums everything in iterator honoring errors
#[allow(clippy::option_if_let_else)] // `map_or` requires `E: Error` to also implement `Copy`
pub fn sum_everything<T, E>(mut items: impl Iterator<Item=Result<T, E>>) -> Result<T, E>
    where T: Default + Copy + Add<Output=T>,
          E: Error,
{
    let sum_result = items
        .try_fold(
            Ok(T::default()),
            |acc, nxt| match nxt {
                Ok(nxt) => ControlFlow::Continue(Ok(acc.unwrap() + nxt)),
                Err(_) => ControlFlow::Break(nxt)
            },
        );

    match sum_result {
        ControlFlow::Continue(ok) => ok,
        ControlFlow::Break(err) => err
    }
}