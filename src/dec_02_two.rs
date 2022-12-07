//! [AOC 2022 Day 2](https://adventofcode.com/2022/day/2)

use std::io;
use std::io::Read;
use std::ops::ControlFlow;
use std::str::FromStr;

use crate::utils::{CleansedLines, io_error};

/// Iterates a file with an encrypted strategy guide that contains
/// the opponent's anticipated move and the outcome you should achieve
struct StrategyGuide<R> {
    lines: CleansedLines<R>,
}

impl<R> StrategyGuide<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input)
        }
    }
}

impl<R> Iterator for StrategyGuide<R>
    where R: Read
{
    type Item = io::Result<(Played, Outcome)>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.lines.next()?.ok()?;

        return Some(strategy(&next));

        fn strategy(play: &str) -> io::Result<(Played, Outcome)> {
            // each play should only contain two symbols, the opponent's play and your strategy
            let (opponent, strategy) = play.split_once(' ')
                .ok_or_else(
                    || io_error(&format!("{play:?} is not a valid play strategy"))
                )?;

            // parse opponent's played move
            let opponent = Played::from_str(opponent)
                .map_err(|_| io_error(&format!("{opponent:?} is not a valid opponent move")))?;

            // parse the strategy you should you
            let strategy = Outcome::from_str(strategy)
                .map_err(|_| io_error(&format!("{strategy:?} is not a valid strategy")))?;

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


impl FromStr for Played {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(match source.trim().to_uppercase().as_str() {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
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

impl FromStr for Outcome {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(match source.trim().to_uppercase().as_str() {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(())
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
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    // calculate total score according to the strategy guide;
    // playing a move that produces the suggested strategy
    let total_score = StrategyGuide::new(input)
        .try_fold(0, |acc, game_strategy| {
            #[allow(clippy::option_if_let_else)] // map_or doesn't work because of move
            if let Ok(game_strategy) = game_strategy {
                let (opponent, outcome) = game_strategy;

                ControlFlow::Continue(Played::from((opponent, outcome)) as usize + outcome as usize + acc)
            } else {
                ControlFlow::Break(game_strategy)
            }
        });

    match total_score {
        ControlFlow::Continue(ok) => Ok(Box::new(ok)),
        ControlFlow::Break(Err(err)) => Err(err),
        ControlFlow::Break(Ok(_)) => unreachable!()
    }
}
