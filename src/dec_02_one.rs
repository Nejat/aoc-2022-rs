use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};

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
    type Item = io::Result<(Game, Game)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines.next()?.ok()?;

        while line.trim().is_empty() {
            line = self.lines.next()?.ok()?;
        }

        return Some(strategy(&line));

        fn strategy(play: &str) -> io::Result<(Game, Game)> {
            let (opponent, you) = play.split_once(' ')
                .ok_or_else(
                    || Error::new(ErrorKind::Other, format!("{play:?} is not a valid strategy"))
                )?;
            let opponent = GameResult::from(opponent).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{opponent:?} is not a valid opponent move")))?;
            let you = GameResult::from(you).0
                .map_err(|_| Error::new(ErrorKind::Other, format!("{you:?} is not a valid move for you")))?;

            Ok((opponent, you))
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct GameResult(Result<Game, ()>);

impl<'a> From<&'a str> for GameResult {
    fn from(source: &'a str) -> GameResult {
        GameResult(match source.trim().to_uppercase().as_str() {
            "A" | "X" => Ok(Game::Rock),
            "B" | "Y" => Ok(Game::Paper),
            "C" | "Z" => Ok(Game::Scissors),
            _ => Err(())
        })
    }
}

pub fn puzzle_one(input: File) -> io::Result<Box<dyn ToString>> {
    const DRAW: usize = 3;
    const LOSE: usize = 0;
    const WIN: usize = 6;

    let total_score = StrategyGuide::new(input)
        .fold(Ok(0), |acc: io::Result<usize>, nxt| {
            if let Ok(acc) = acc {
                let (opponent, you) = nxt?;

                Ok(
                    match (opponent, you) {
                        (Game::Rock, Game::Paper) |
                        (Game::Paper, Game::Scissors) |
                        (Game::Scissors, Game::Rock) => WIN,

                        (Game::Rock, Game::Rock) |
                        (Game::Paper, Game::Paper) |
                        (Game::Scissors, Game::Scissors) => DRAW,

                        (Game::Rock, Game::Scissors) |
                        (Game::Paper, Game::Rock) |
                        (Game::Scissors, Game::Paper) => LOSE,
                    }
                        + (you as usize)
                        + acc
                )
            } else {
                acc
            }
        })
        .map(Box::new)?;

    Ok(total_score)
}
