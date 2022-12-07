//! [AOC 2022 Day 2](https://adventofcode.com/2022/day/2)

use std::io;
use std::io::Read;
use std::ops::ControlFlow;
use std::str::FromStr;

use crate::utils::{CleansedLines, io_error};

struct StrategyGuide<R> {
    lines: CleansedLines<R>,
}

/// Iterates a file with an encrypted strategy guide that contains
/// the opponent's anticipated move and the move you should play
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
    type Item = io::Result<(Played, Played)>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.lines.next()?.ok()?;

        return Some(strategy(&next));

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
        .try_fold(0, |acc, game_strategy| {
            #[allow(clippy::option_if_let_else)] // map_or doesn't work because of move
            if let Ok(game_strategy) = game_strategy {
                let (opponent, you) = game_strategy;

                ControlFlow::Continue(Outcome::from((opponent, you)) as usize + you as usize + acc)
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
