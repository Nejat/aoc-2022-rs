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
    type Item = io::Result<(Played, Outcome)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines.next()?.ok()?;

        // read file line by line skipping empty lines
        while line.trim().is_empty() {
            line = self.lines.next()?.ok()?;
        }

        return Some(strategy(&line));

        fn strategy(play: &str) -> io::Result<(Played, Outcome)> {
            // each play should only contain two symbols, the opponent's play and your strategy
            let (opponent, strategy) = play.split_once(' ')
                .ok_or_else(
                    || Error::new(ErrorKind::Other, format!("{play:?} is not a valid play strategy"))
                )?;

            // parse opponent's played move
            let opponent = PlayedDecrypted::from(opponent).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{opponent:?} is not a valid opponent move")))?;

            // parse the strategy you should you
            let strategy = OutcomeDecrypted::from(strategy).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{strategy:?} is not a valid strategy")))?;

            Ok((opponent, strategy))
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum Played {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct PlayedDecrypted(Result<Played, ()>);

impl<'a> From<&'a str> for PlayedDecrypted {
    fn from(source: &'a str) -> Self {
        Self(match source.trim().to_uppercase().as_str() {
            "A" => Ok(Played::Rock),
            "B" => Ok(Played::Paper),
            "C" => Ok(Played::Scissors),
            _ => Err(())
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Outcome {
    Draw = 3,
    Lose = 0,
    Win = 6,
}

struct OutcomeDecrypted(Result<Outcome, ()>);

impl<'a> From<&'a str> for OutcomeDecrypted {
    fn from(source: &'a str) -> Self {
        Self(match source.trim().to_uppercase().as_str() {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(())
        })
    }
}

/// Determine what to play for a desired outcome of an anticipated move played
impl From<(Self, Outcome)> for Played {
    fn from((played, desired_outcome): (Self, Outcome)) -> Self {
        match (played, desired_outcome) {
            (Self::Rock, Outcome::Draw) |
            (Self::Paper, Outcome::Lose) |
            (Self::Scissors, Outcome::Win) => Self::Rock,

            (Self::Paper, Outcome::Draw) |
            (Self::Scissors, Outcome::Lose) |
            (Self::Rock, Outcome::Win) => Self::Paper,

            (Self::Scissors, Outcome::Draw) |
            (Self::Rock, Outcome::Lose) |
            (Self::Paper, Outcome::Win) => Self::Scissors,
        }
    }
}

/// Play Rock, Paper, Scissors assuming the strategy guide is encrypted as the outcome of playing
pub fn puzzle_two(input: File) -> io::Result<Box<dyn ToString>> {
    // calculate total score according to the strategy guide;
    // playing a move that produces the suggested strategy
    let total_score = StrategyGuide::new(input)
        .fold(Ok(0), |acc: io::Result<usize>, game_strategy| {
            if let Ok(acc) = acc {
                let (opponent, outcome) = game_strategy?;

                Ok(Played::from((opponent, outcome)) as usize + outcome as usize + acc)
            } else {
                acc
            }
        })
        .map(Box::new)?;

    Ok(total_score)
}
