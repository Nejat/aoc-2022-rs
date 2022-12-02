use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};

struct StrategyGuide {
    lines: Lines<BufReader<File>>,
}

/// Iterates a file with an encrypted strategy guide that contains
/// the opponent's anticipated move and the move you should play
impl StrategyGuide {
    fn new(input: File) -> Self {
        Self {
            lines: BufReader::new(input).lines()
        }
    }
}

impl Iterator for StrategyGuide {
    type Item = io::Result<(Played, Played)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines.next()?.ok()?;

        // read file line by line skipping empty lines
        while line.trim().is_empty() {
            line = self.lines.next()?.ok()?;
        }

        return Some(strategy(&line));

        fn strategy(play: &str) -> io::Result<(Played, Played)> {
            // each play should only contain two symbols, the opponent's play and your play
            let (opponent, you) = play.split_once(' ')
                .ok_or_else(
                    || Error::new(ErrorKind::Other, format!("{play:?} is not a valid strategy"))
                )?;

            // parse opponent's played move
            let opponent = PlayedDecrypted::from(opponent).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{opponent:?} is not a valid opponent move")))?;

            // parse the move you should play
            let you = PlayedDecrypted::from(you).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{you:?} is not a valid move for you")))?;

            Ok((opponent, you))
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

struct PlayedDecrypted(Result<Played, ()>);

impl<'a> From<&'a str> for PlayedDecrypted {
    fn from(source: &'a str) -> Self {
        Self(match source.trim().to_uppercase().as_str() {
            "A" | "X" => Ok(Played::Rock),
            "B" | "Y" => Ok(Played::Paper),
            "C" | "Z" => Ok(Played::Scissors),
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

/// Determine outcome of a game played (opponent, you)
impl From<(Played, Played)> for Outcome {
    fn from((opponent, you): (Played, Played)) -> Self {
        match (opponent, you) {
            (Played::Rock, Played::Paper) |
            (Played::Paper, Played::Scissors) |
            (Played::Scissors, Played::Rock) => Self::Win,

            (Played::Rock, Played::Rock) |
            (Played::Paper, Played::Paper) |
            (Played::Scissors, Played::Scissors) => Self::Draw,

            (Played::Rock, Played::Scissors) |
            (Played::Paper, Played::Rock) |
            (Played::Scissors, Played::Paper) => Self::Lose,
        }
    }
}

/// Play Rock, Paper, Scissors assuming the strategy guide is encrypted as moves you should play
pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
    // calculate total score according to the strategy guide; playing the suggested moves
    let total_score = StrategyGuide::new(input)
        .fold(Ok(0), |acc: io::Result<usize>, game_strategy| {
            if let Ok(acc) = acc {
                let (opponent, you) = game_strategy?;

                Ok(Outcome::from((opponent, you)) as usize + you as usize + acc)
            } else {
                acc
            }
        })
        .map(Box::new)?;

    Ok(total_score)
}
