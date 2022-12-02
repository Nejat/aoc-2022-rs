use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};

/// Iterates a file with an encrypted strategy guide that contains
/// the opponent's anticipated move and the outcome you should achieve
struct StrategyGuide {
    lines: Lines<BufReader<File>>,
}

impl StrategyGuide {
    fn new(input: File) -> Self {
        Self {
            lines: BufReader::new(input).lines()
        }
    }
}

impl Iterator for StrategyGuide {
    type Item = io::Result<(Played, Strategy)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines.next()?.ok()?;

        // read file line by line skipping empty lines
        while line.trim().is_empty() {
            line = self.lines.next()?.ok()?;
        }

        return Some(strategy(&line));

        fn strategy(play: &str) -> io::Result<(Played, Strategy)> {
            // each play should only contain two symbols, the opponent's play and your strategy
            let (opponent, strategy) = play.split_once(' ')
                .ok_or_else(
                    || Error::new(ErrorKind::Other, format!("{play:?} is not a valid play strategy"))
                )?;

            // parse opponent's played move
            let opponent = PlayedResult::from(opponent).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{opponent:?} is not a valid opponent move")))?;

            // parse the strategy you should you
            let strategy = StrategyResult::from(strategy).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{strategy:?} is not a valid strategy")))?;

            Ok((opponent, strategy))
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Played {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct PlayedResult(Result<Played, ()>);

impl<'a> From<&'a str> for PlayedResult {
    fn from(source: &'a str) -> Self {
        Self(match source.trim().to_uppercase().as_str() {
            "A" => Ok(Played::Rock),
            "B" => Ok(Played::Paper),
            "C" => Ok(Played::Scissors),
            _ => Err(())
        })
    }
}

#[derive(Clone, Copy)]
enum Strategy {
    Draw,
    Lose,
    Win,
}

struct StrategyResult(Result<Strategy, ()>);

impl<'a> From<&'a str> for StrategyResult {
    fn from(source: &'a str) -> Self {
        Self(match source.trim().to_uppercase().as_str() {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(())
        })
    }
}

/// Play Rock, Paper, Scissors assuming the strategy guide is encrypted as the outcome of playing
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    const DRAW: usize = 3;
    const LOSE: usize = 0;
    const WIN: usize = 6;

    // calculate total score according to the strategy guide;
    // playing a move that produces the suggested strategy
    let total_score = StrategyGuide::new(input)
        .fold(Ok(0), |acc: io::Result<usize>, nxt| {
            if let Ok(acc) = acc {
                let (opponent, you) = nxt?;

                Ok(
                    match (opponent, you) {
                        (Played::Rock, Strategy::Draw) => Played::Rock as usize + DRAW,
                        (Played::Scissors, Strategy::Draw) => Played::Scissors as usize + DRAW,
                        (Played::Paper, Strategy::Draw) => Played::Paper as usize + DRAW,
                        (Played::Rock, Strategy::Lose) => Played::Scissors as usize + LOSE,
                        (Played::Paper, Strategy::Lose) => Played::Rock as usize + LOSE,
                        (Played::Scissors, Strategy::Lose) => Played::Paper as usize + LOSE,
                        (Played::Rock, Strategy::Win) => Played::Paper as usize + WIN,
                        (Played::Paper, Strategy::Win) => Played::Scissors as usize + WIN,
                        (Played::Scissors, Strategy::Win) => Played::Rock as usize + WIN,
                    }
                        + acc
                )
            } else {
                acc
            }
        })
        .map(Box::new)?;

    Ok(total_score)
}
