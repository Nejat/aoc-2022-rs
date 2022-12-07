//! [AOC 2022 Day 2](https://adventofcode.com/2022/day/2)

use std::io;
use std::io::{BufRead, BufReader, Lines, Read};
use std::str::FromStr;

use crate::utils::io_error;

struct StrategyGuide<R> {
    lines: Lines<BufReader<R>>,
}

/// Iterates a file with an encrypted strategy guide that contains
/// the opponent's anticipated move and the move you should play
impl<R> StrategyGuide<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: BufReader::new(input).lines()
        }
    }
}

impl<R> Iterator for StrategyGuide<R>
    where R: Read
{
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
                    || io_error(&format!("{play:?} is not a valid strategy"))
                )?;

            // parse opponent's played move
            let opponent = Played::from_str(opponent)
                .map_err(|_| io_error(&format!("{opponent:?} is not a valid opponent move")))?;

            // parse the move you should play
            let you = Played::from_str(you)
                .map_err(|_| io_error(&format!("{you:?} is not a valid move for you")))?;

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

impl FromStr for Played {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(match src.trim().to_uppercase().as_str() {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => return Err(())
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
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
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
