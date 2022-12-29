//! [AOC 2022 Day 8](https://adventofcode.com/2022/day/8)

use std::io;
use std::io::{BufRead, BufReader, Read};
use std::ops::ControlFlow;

/// find all visible trees from outside the grid
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let (width, height, trees) = parse_forrest(input)?;
    let mut hidden = Vec::new();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let tree = trees.get(x + y * width).unwrap();

            if (0..y).any(|y| trees.get(x + y * width).unwrap() >= tree) &&
                (y + 1..height).any(|y| trees.get(x + y * width).unwrap() >= tree) &&
                (0..x).any(|x| trees.get(x + y * width).unwrap() >= tree) &&
                (x + 1..width).any(|x| trees.get(x + y * width).unwrap() >= tree) {
                hidden.push(((x, y), *tree));
            }
        }
    }

    Ok(Box::new(width * height - hidden.len()))
}

/// finds highest scenic score possible for any tree
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let (width, height, forrest) = parse_forrest(input)?;
    let mut most_scenic = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let candidate = forrest.get(x + y * width).unwrap();

            let north = (0..y).map(|y1| x + (y - 1 - y1) * width);
            let north = gaze_upon_yonder(north, *candidate, &forrest);

            let south = (y + 1..height).map(|y| x + y * width);
            let south = gaze_upon_yonder(south, *candidate, &forrest);

            let east = (x + 1..width).map(|x| x + y * width);
            let east = gaze_upon_yonder(east, *candidate, &forrest);

            let west = (0..x).map(|x1| (x - 1 - x1) + y * width);
            let west = gaze_upon_yonder(west, *candidate, &forrest);

            let rank = north * south * west * east;

            most_scenic = most_scenic.max(rank);
        }
    }

    Ok(Box::new(most_scenic))
}

/// calculates the scenic score of a single gaze direction
fn gaze_upon_yonder(
    mut yonder: impl Iterator<Item=usize>,
    candidate: u8,
    forrest: &[u8],
) -> usize {
    let score = yonder
        .try_fold(0, |score, tree_in_forrest| {
            let checked_tree = *forrest.get(tree_in_forrest).unwrap();

            if checked_tree >= candidate {
                ControlFlow::Break(score + 1)
            } else {
                ControlFlow::Continue(score + 1)
            }
        });

    let score = match score {
        ControlFlow::Continue(score) |
        ControlFlow::Break(score) => score
    };

    score.max(1)
}

/// parses an input file of s planted forrest and it's width & height
fn parse_forrest<R>(input: R) -> io::Result<(usize, usize, Vec<u8>)>
    where R: Read
{
    let mut width = 0;
    let mut height = 0;
    let zero = b'0';

    let trees = BufReader::new(input)
        .lines()
        .try_fold(
            Vec::new(),
            |mut forrest, row| {
                let mut row = match row {
                    Ok(row) => row.chars().map(|c| c as u8 - zero).collect::<Vec<_>>(),
                    Err(_) => return ControlFlow::Break(row)
                };

                width = width.max(row.len());
                height += 1;

                forrest.append(&mut row);

                ControlFlow::Continue(forrest)
            });

    match trees {
        ControlFlow::Continue(trees) => Ok((width, height, trees)),
        ControlFlow::Break(Err(err)) => Err(err),
        ControlFlow::Break(Ok(_)) => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn puzzle_one() {
        let expected = "21";
        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "8";
        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}
