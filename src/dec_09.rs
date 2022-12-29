//! [AOC 2022 Day 9](https://adventofcode.com/2022/day/9)

use std::{fmt, io};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::ops::{Add, Deref};
use std::str::FromStr;

use crate::utils::{CleansedLines, io_error};

const TRACE_SOLUTION: bool = true;

/// Iterator of head movements
struct FollowYourHead<R> {
    /// input stream of head movements
    lines: CleansedLines<R>,
    /// current location of head
    current: Option<Location>,
    /// step iterator of current head movement
    steps: Option<Box<dyn Iterator<Item=Movement>>>,
}

impl<R> FollowYourHead<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
            current: None,
            steps: None,
        }
    }
}

impl<R> Iterator for FollowYourHead<R>
    where R: Read
{
    type Item = io::Result<Location>;

    fn next(&mut self) -> Option<Self::Item> {
        // at start; initialize
        if self.current.is_none() {
            self.current = Some(Location::default());

            return Some(Ok(Location::default()))
        }

        #[allow(unused_assignments)] // is reassigned before reading
        let mut movement = Movement::None;

        loop {
            // if no more steps, get next head movement
            if self.steps.is_none() {
                let next = self.lines.next()?.ok()?;
                let next_steps = parse_move(&next).ok()?;

                // setup step iterator
                self.steps = Some(Box::new(next_steps));
            }

            // iterate next step
            if self.steps.as_mut().and_then(|stepper|
                stepper.next().map(|next| { movement = next; })
            ).is_some() {
                // break if has next step
                break;
            }

            // otherwise, clear empty steps
            self.steps = None;
        }

        // get move step value of movement
        let move_step = &*movement;
        // calculate next head current location with next move step
        let current = self.current.expect("expect a current head location") + move_step;

        // update head current location
        self.current = Some(current);

        return Some(Ok(current));

        // parse steps instruction input, result err match iterator Item
        fn parse_move(input: &str) -> Result<Movement, Option<io::Result<Location>>> {
            match input.parse::<Movement>() {
                Ok(steps) => Ok(steps),
                Err(err) => Err(Some(Err(io_error(&err))))
            }
        }
    }
}

/// represents a size value of height and width
#[derive(Copy, Clone, Debug, Default)]
struct Size {
    height: usize,
    width: usize,
}

/// represents a location x, y
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
struct Location {
    x: isize,
    y: isize,
}

impl Display for Location {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

/// represents a movement vector
#[derive(Copy, Clone, Debug, Default)]
struct Move {
    x: isize,
    y: isize,
}

/// add operator for adding a move vector to location
impl Add<&Move> for Location {
    type Output = Self;

    fn add(self, rhs: &Move) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

/// Grid setup information
#[derive(Copy, Clone, Default)]
struct GridSetup {
    /// size of grid
    size: Size,
    /// start location for grid
    start: Option<Location>,
}

impl GridSetup {
    /// return a start location, chooses the center if one is not provided
    #[allow(clippy::cast_possible_wrap)]
    fn start(&mut self) -> Location {
        if self.start.is_none() {
            self.start = Some(Location {
                x: (self.size.width >> 1) as isize,
                y: (self.size.height >> 1) as isize,
            });
        }

        self.start.expect("expect a start location is defined")
    }
}

/// Iterator of final tail knot movements
struct FollowYourTail<R> {
    follow: FollowYourHead<R>,
    grid_setup: Option<GridSetup>,
    knots: Vec<Location>,
    trace: bool,
}

impl<R> FollowYourTail<R>
    where R: Read
{
    fn new(input: R, knots: usize, grid_setup: Option<GridSetup>, trace: bool) -> Self {
        let knots = knots.min(9);
        let knots_range = 0..=knots;
        let start = Location::default();
        let mut knots = Vec::with_capacity(knots + 1);

        knots_range.for_each(|_| knots.push(start));

        Self {
            follow: FollowYourHead::new(input),
            grid_setup,
            knots,
            trace,
        }
    }
}

impl<R> Iterator for FollowYourTail<R>
    where R: Read
{
    type Item = io::Result<Location>;

    fn next(&mut self) -> Option<Self::Item> {
        const HEAD_TAIL: &str = "HT";
        const KNOTS: &str = "H123456789";
        const VALID_SYMBOL: &str = "expect a valid symbol to draw";

        // requires at least 2 knots; a Head & 1 Tail knot
        if self.knots.len() <= 1 {
            return None;
        }

        // choose the correct symbol provider based on the number of knots
        let get_symbol = if self.knots.len() == 2 {
            |idx| HEAD_TAIL.chars().nth(idx).expect(VALID_SYMBOL)
        } else {
            |idx| KNOTS.chars().nth(idx).expect(VALID_SYMBOL)
        };

        // first gets where the head's next movement is
        let mut next = self.follow.next()?.ok()?;
        // define range of knots to iterate, one less than total knots
        let knots_range = 0..self.knots.len() - 1;
        let knots = self.knots.as_mut_slice();

        // optionally setup of a path grid and symbols tracker
        let mut path = self.grid_setup.map(Grid::new);
        // symbols need to be tracked in the opposite order than they are processed
        let mut symbols = self.grid_setup.map(|_| VecDeque::new());

        if self.trace {
            println!("N: {next}");
        }

        // iterator over knot range
        for next_knot in knots_range {
            let head = knots[next_knot];
            let tail = knots[next_knot + 1];

            // get next tail location, based on next head location
            let next_tail = next_tail_location(tail, next);

            if self.trace {
                println!("T{}: {tail}, NT: {next_tail}, H: {head}, NH: {next}", next_knot + 1);
            }

            // if a grid is being output, store head symbol and locations
            symbols.as_mut().and_then(|symbols| {
                symbols.push_front((head, next, get_symbol(next_knot)));
                <Option<()>>::None
            });

            // if a grid is being output, store tail symbol and locations
            symbols.as_mut().and_then(|symbols| {
                symbols.push_front((tail, next_tail, get_symbol(next_knot + 1)));
                <Option<()>>::None
            });

            // update current head knot location
            knots[next_knot] = next;
            // update current tail knot location
            knots[next_knot + 1] = next_tail;
            // new tail location is next head location
            next = next_tail;
        }

        // if grid is being displayed update grid with stored locations and symbols
        symbols.and_then(|symbols| {
            let path = path.as_mut()?;

            for (prior, next, symbol) in symbols {
                path.update(&prior, &next, symbol);
            }

            <Option<()>>::None
        });

        if let Some(path) = path {
            println!("\n{}\n", path.to_string());
        }

        // get the visited location of last knot
        let visited = knots.last().expect("expected at least one tail");

        if self.trace {
            println!("V: {visited}\n");
        }

        return Some(Ok(*visited));

        fn next_tail_location(
            tail: Location,
            head: Location,
        ) -> Location {
            // defined the bound box to determine of the head is within reach of tail
            let bounding_width = tail.x - 1..=tail.x + 1;
            let bounding_height = tail.y - 1..=tail.y + 1;

            if bounding_width.contains(&head.x) && bounding_height.contains(&head.y) {
                // if with reach don't change tail
                tail
            } else {
                // new tail x offset, movement by max one movement in x direction
                let offset_x = if head.x > tail.x {
                    (head.x - tail.x).min(1)
                } else {
                    -(tail.x - head.x).min(1)
                };

                // new tail y offset, movement by max one movement in y direction
                let offset_y = if head.y > tail.y {
                    (head.y - tail.y).min(1)
                } else {
                    -(tail.y - head.y).min(1)
                };

                Location {
                    x: tail.x + offset_x,
                    y: tail.y + offset_y,
                }
            }
        }
    }
}

/// a set of movement instructions
#[derive(Copy, Clone)]
enum Movement {
    None,
    Down(usize),
    Left(usize),
    Right(usize),
    Up(usize),
}

// translation of movement instruction, one step at a time
impl Movement {
    const MOVE_DOWN: Move = Move { x: 0, y: -1 };
    const MOVE_LEFT: Move = Move { x: -1, y: 0 };
    const MOVE_RIGHT: Move = Move { x: 1, y: 0 };
    const MOVE_UP: Move = Move { x: 0, y: 1 };
    const DONT_MOVE: Move = Move { x: 0, y: 0 };
}

/// parse movement instruction from string
impl FromStr for Movement {
    type Err = String;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = src.split_once(' ')
            .ok_or_else(|| format!("'{src} is not a valid move"))?;

        let steps = steps.trim()
            .parse::<usize>()
            .map_err(|err| format!(
                "'{}' is not a valid steps value - {err}", steps.trim()
            ))? + 1;

        Ok(match dir.trim() {
            "D" | "d" => Self::Down(steps),
            "L" | "l" => Self::Left(steps),
            "R" | "r" => Self::Right(steps),
            "U" | "u" => Self::Up(steps),
            _ => return Err(format!("'{}' is not a valid direction", dir.trim())),
        })
    }
}

/// iterate movement one step at a time
#[allow(clippy::copy_iterator)]
impl Iterator for Movement {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Down(steps) if *steps > 1 => {
                *steps -= 1;

                Some(*self)
            },
            Self::Left(steps) if *steps > 1 => {
                *steps -= 1;

                Some(*self)
            },
            Self::Right(steps) if *steps > 1 => {
                *steps -= 1;

                Some(*self)
            },
            Self::Up(steps) if *steps > 1 => {
                *steps -= 1;

                Some(*self)
            },
            _ => None
        }
    }
}

// translate movement step
impl Deref for Movement {
    type Target = Move;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Down(steps) if *steps > 0 => &Self::MOVE_DOWN,
            Self::Left(steps) if *steps > 0 => &Self::MOVE_LEFT,
            Self::Right(steps) if *steps > 0 => &Self::MOVE_RIGHT,
            Self::Up(steps) if *steps > 0 => &Self::MOVE_UP,
            _ => &Self::DONT_MOVE
        }
    }
}

/// display grid for current state of calculation, for debugging
struct Grid {
    grid: Vec<u8>,
    start: Location,
    width: isize,
}

impl Grid {
    #[allow(clippy::cast_possible_wrap)]
    fn new(mut grid_setup: GridSetup) -> Self {
        let size = grid_setup.size;
        // initial empty grid
        let grid = (0..size.height).map(|_| ".".repeat(size.width)).collect::<Vec<_>>().join("\n");
        let start = grid_setup.start();
        let width = size.width + 1;

        Self {
            grid: grid.into_bytes(),
            start,
            width: width as isize,
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.grid).to_string()
    }
}

impl Grid {
    /// update a next location w/a symbol
    fn update(
        &mut self,
        prior: &Location,
        next: &Location,
        symbol: char,
    ) {
        // transpose prior location relative to start and get index into grid
        let idx = self.index(&self.transpose(prior));
        let symbol = symbol as u8;

        // if index has same value, erase it
        if self.grid[idx] == symbol {
            self.grid[idx] = b'.';
        }

        // get index of start location
        let start = self.index(&self.start);

        // if index is blank or has been erased, draw start indicator
        if self.grid[start] == b'.' {
            self.grid[start] = b's';
        }

        // transpose next location relative to start and get index into grid
        let idx = self.index(&self.transpose(next));

        // draw symbol
        self.grid[idx] = symbol;
    }

    /// draw visited locations to grid
    #[allow(dead_code)] // used during dev testing
    fn visited(&mut self, visited: impl Iterator<Item=Location>)
    {
        for location in visited {
            let idx = self.index(&self.transpose(&location));

            self.grid[idx] = b'#';
        }
    }

    /// transpose a location relative to grid start location
    const fn transpose(&self, location: &Location) -> Location {
        Location { x: location.x + self.start.x, y: self.start.y - location.y }
    }

    /// get the index of a flat grid representation from a 2d location
    #[allow(clippy::cast_sign_loss)]
    const fn index(&self, location: &Location) -> usize {
        (location.x + location.y * self.width) as usize
    }
}

/// solution for all locations visited by one knot
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    puzzle_do(input, 1, None, !TRACE_SOLUTION)
}

/// solution for all locations visited by last knot of nine knots
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    puzzle_do(input, 9, None, !TRACE_SOLUTION)
}

fn puzzle_do<R>(
    input: R,
    knots: usize,
    grid_setup: Option<GridSetup>,
    trace: bool,
) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let visited = FollowYourTail::new(input, knots, grid_setup, trace)
        .collect::<Result<HashSet<_>, _>>()?;
    let solution = visited.len();

    if let Some(grid_setup) = grid_setup {
        let mut path = Grid::new(grid_setup);

        path.visited(visited.into_iter());

        println!("\n{}\n", path.to_string());
    }

    Ok(Box::new(solution))
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    #[test]
    fn puzzle_one() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let expected = "13";

        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        // let grid_setup = super::GridSetup {
        //     size: super::Size { height: 5, width: 7 },
        //     start: Some(super::Location { x: 0, y: 4 }),
        // };
        //
        // let actual = super::puzzle_do(INPUT.as_bytes(), 1, Some(grid_setup), !super::TRACE_SOLUTION)
        //     .expect(EXPECTED_PUZZLE_SOLUTION)
        //     .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let expected = "36";

        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        // let grid_setup = super::GridSetup {
        //     size: super::Size { height: 30, width: 30 },
        //     start: None,
        // };
        //
        // let actual = super::puzzle_do(INPUT.as_bytes(), 9, Some(grid_setup), super::TRACE_SOLUTION)
        //     .expect(EXPECTED_PUZZLE_SOLUTION)
        //     .to_string();

        assert_eq!(actual, expected);
    }
}
