//! [AOC 2022 Day 11](https://adventofcode.com/2022/day/11)

extern crate nom;

use std::{fmt, io};
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::marker::PhantomData;
use std::ops::{Add, Mul};
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::IResult;
use nom::multi::separated_list1;
use num_traits::Num;

use crate::utils::{CleansedLines, io_error};

// for the second part of the puzzle I made the notes parser generic for the number type
// used for calculating worry values, so that I can use 64 bit floating point values.
// as it turned out that was not the correct solution, but it was a good learning exercise
// so I kept it in

/// iterator of notes of monkeys
/// generic type `N` was necessary to make iterator of
/// generic `Monkey<N>` types
struct Notes<R, N> {
    lines: CleansedLines<R>,
    phantom: PhantomData<N>,
}

impl<R, N> Notes<R, N>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
            phantom: PhantomData::default(),
        }
    }
}

impl<R, N> Iterator for Notes<R, N>
    where R: Read,
          N: Num + FromStr + Display,
          <N as FromStr>::Err: Debug + Display
{
    type Item = io::Result<Monkey<N>>;

    fn next(&mut self) -> Option<Self::Item> {
        // parses Monkey notes lazily
        let (_, monkey_id) = match parse_monkey_id(&self.lines.next()?.ok()?) {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid money identifier: {err}"))))
        };

        let (_, start_items) = match parse_start_items(&self.lines.next()?.ok()?) {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid start items: {err}"))))
        };

        let (_, operation) = match parse_operation(&self.lines.next()?.ok()?) {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid operation: {err}"))))
        };

        let (_, test) = match parse_test(&self.lines.next()?.ok()?) {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid test: {err}"))))
        };

        let (_, true_throw) = match parse_decision(&self.lines.next()?.ok()?, "true") {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid operation: {err}"))))
        };

        let (_, false_throw) = match parse_decision(&self.lines.next()?.ok()?, "false") {
            Ok(parsed) => parsed,
            Err(err) => return Some(Err(io_error(&format!("invalid operation: {err}"))))
        };

        return Some(Ok(Monkey {
            id: monkey_id,
            items: start_items,
            worried: operation,
            test,
            throws: [true_throw, false_throw],
        }));

        // parses the decision for a monkey; from notes
        fn parse_decision<'a>(input: &'a str, result: &'a str) -> IResult<&'a str, usize> {
            let (next, _) = space1(input)?;
            let (next, _) = tag("If")(next)?;
            let (next, _) = space1(next)?;
            let (next, _) = tag(result)(next)?;
            let (next, _) = tag(": throw to monkey")(next)?;
            let (next, _) = space1(next)?;
            let (next, monkey) = digit1(next)?;

            Ok((next, monkey.parse().expect("")))
        }

        // parses the monkey identifier; from notes
        fn parse_monkey_id(input: &str) -> IResult<&str, usize> {
            let (next, _) = tag("Monkey")(input)?;
            let (next, _) = space1(next)?;
            let (next, id) = parse_number(next)?;
            let (next, _) = tag(":")(next)?;

            Ok((next, id))
        }

        // parses the monkey's operation; from notes
        fn parse_operation<N>(input: &str) -> IResult<&str, Operation<N>>
            where N: Num + FromStr + Display,
                  <N as FromStr>::Err: Debug + Display
        {
            let (next, _) = space1(input)?;
            let (next, _) = tag("Operation: new =")(next)?;
            let (operation, _) = space1(next)?;

            Ok(("", Operation::from_str(operation).expect("")))
        }

        // parses starting items a monkey has; from notes
        fn parse_start_items<N>(input: &str) -> IResult<&str, VecDeque<N>>
            where N: Num + FromStr + Display,
                  <N as FromStr>::Err: Debug + Display
        {
            let (next, _) = space1(input)?;
            let (next, _) = tag("Starting items:")(next)?;
            let (next, _) = space1(next)?;
            let (next, list) = separated_list1(tag(", "), parse_number)(next)?;

            Ok((next, VecDeque::from(list)))
        }

        // parses the monkey's observed test; from notes
        fn parse_test<N>(input: &str) -> IResult<&str, N>
            where N: Num + FromStr + Display,
                  <N as FromStr>::Err: Debug + Display
        {
            let (next, _) = space1(input)?;
            let (next, _) = tag("Test: divisible by")(next)?;
            let (next, _) = space1(next)?;
            let (next, test) = digit1(next)?;

            Ok((next, test.parse().expect("")))
        }

        // parses a whole number as `N`
        fn parse_number<N>(input: &str) -> IResult<&str, N>
            where N: Num + FromStr + Display,
                  <N as FromStr>::Err: Debug + Display
        {
            let (next, value) = digit1(input)?;

            Ok((next, value.parse().expect("")))
        }
    }
}

/// Notes of a particular monkey
#[derive(Debug)]
#[allow(dead_code)]
struct Monkey<N>
    where N: Display
{
    /// the identifier of the monkey observed
    id: usize,
    /// items the monkey starts with
    items: VecDeque<N>,
    /// the worry operation of the monkey
    worried: Operation<N>,
    /// the test the monkey uses to decide where to throw the current item
    test: N,
    /// the two monkeys that are thrown to, based on the worry test the monkey determines
    throws: [usize; 2],
}

/// the worry operations a monkey calculates
enum Operation<N> {
    ///  adds two values
    Add {
        lhs: Operand<N>,
        rhs: Operand<N>,
    },
    // multiplies two values
    Multiply {
        lhs: Operand<N>,
        rhs: Operand<N>,
    },
}

impl<N> Debug for Operation<N>
    where N: Debug + Display
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { lhs, rhs } => write!(fmt, "{lhs:?} + {rhs:?}"),
            Self::Multiply { lhs, rhs } => write!(fmt, "{lhs:?} * {rhs:?}"),
        }
    }
}

impl<N> Operation<N>
    where N: Copy + Add<Output=N> + Mul<Output=N>
{
    /// calculation the monkey performs before testing your worry level
    fn calc(&self, old: N) -> N {
        match self {
            Self::Add { lhs, rhs } => lhs.value(old) + rhs.value(old),
            Self::Multiply { lhs, rhs } => lhs.value(old) * rhs.value(old),
        }
    }
}

impl<N> FromStr for Operation<N>
    where N: Num + FromStr + Display,
          <N as FromStr>::Err: Display
{
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (next, lhs) = parse_operand(input).map_err(|err| format!("'{input}' is not a valid operand; {err}"))?;
        let (next, operator) = parse_operator(next).map_err(|err| format!("'{input}' is not a valid operator; {err}"))?;
        let (_, rhs) = parse_operand(next).map_err(|err| format!("'{input}' is not a valid operand; {err}"))?;

        let lhs = lhs.parse().map_err(|err| format!("'{lhs}' is not a valid operand; {err}"))?;
        let rhs = rhs.parse().map_err(|err| format!("'{rhs}' is not a valid operand; {err}"))?;

        return Ok(if operator == "+" {
            Self::Add { lhs, rhs }
        } else {
            Self::Multiply { lhs, rhs }
        });

        // parses the operand `old` or a number
        fn parse_operand(input: &str) -> IResult<&str, &str> {
            alt((alpha1, digit1))(input)
        }

        // parses the operator a `+` or a `*`
        fn parse_operator(input: &str) -> IResult<&str, &str> {
            let (next, _) = space1(input)?;
            let (next, operator) = alt((tag("*"), tag("+")))(next)?;
            let (next, _) = space1(next)?;

            Ok((next, operator))
        }
    }
}

/// operand of monkeys worry operation
#[derive(Copy, Clone)]
enum Operand<N> {
    /// the old value
    Old,
    /// a number of type `N`
    Num(N),
}

impl<N> Debug for Operand<N>
    where N: Display
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Old => write!(fmt, "old"),
            Self::Num(value) => write!(fmt, "{value}")
        }
    }
}

impl<N> Operand<N>
    where N: Copy
{
    /// gets the value of the operand
    const fn value(&self, old: N) -> N {
        match self {
            Self::Old => old,
            Self::Num(value) => *value
        }
    }
}

impl<N> FromStr for Operand<N>
    where N: FromStr + Display,
          <N as FromStr>::Err: Display
{
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.trim().eq_ignore_ascii_case("old") {
            Ok(Self::Old)
        } else {
            Ok(Self::Num(input.parse().map_err(|err| format!("'{input}' is an invalid operand; {err}"))?))
        }
    }
}

const EXPECT_A_MONKEY: &str = "expected a note about a monkey";

pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    let mut monkeys = Notes::new(input).collect::<Result<Vec<Monkey<usize>>, _>>()
        .map_err(|err| io_error(&format!("invalid notes of monkeys; {err}")))?;
    let mut inspected = (0..monkeys.len()).map(|_| 0_usize).collect::<Vec<_>>();

    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let mut queue = Vec::new();
            let monkey: &mut _ = monkeys.get_mut(monkey_idx).expect(EXPECT_A_MONKEY);
            let items = monkey.items.borrow_mut();

            for item in items {
                let worry = monkey.worried.calc(*item) / 3;

                let next_monkey = if worry % monkey.test == 0 {
                    monkey.throws[0]
                } else {
                    monkey.throws[1]
                };

                queue.push((next_monkey, worry));

                *inspected.get_mut(monkey_idx).expect(EXPECT_A_MONKEY) += 1;
            }

            monkey.items.clear();

            for (next_monkey, item) in queue {
                let monkey = monkeys.get_mut(next_monkey).expect(EXPECT_A_MONKEY);

                monkey.items.push_back(item);
            }
        }
    }

    inspected.sort_unstable();
    inspected.reverse();

    let monkey_business: usize = inspected.iter().take(2).product();

    Ok(Box::new(monkey_business))
}

pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    // todo: figure out why f64 does not produce the expected answer
    let mut monkeys = Notes::new(input).collect::<Result<Vec<Monkey<usize>>, _>>()
        .map_err(|err| io_error(&format!("invalid notes of monkeys; {err}")))?;
    let mut inspected = (0..monkeys.len()).map(|_| 0_usize).collect::<Vec<_>>();

    for _round in 1..=10000 {
        // I came close but this is not my answer 😞, https://erri120.github.io/posts/2022-12-11/
        let product = monkeys.iter().fold(1, |acc, nxt| acc * nxt.test);

        for monkey_idx in 0..monkeys.len() {
            let mut queue = Vec::new();
            let monkey = monkeys.get_mut(monkey_idx).expect(EXPECT_A_MONKEY);
            let items = monkey.items.borrow_mut();

            for item in items {
                let worry = monkey.worried.calc(*item) % product;

                let next_monkey = if worry % monkey.test == 0 {
                    monkey.throws[0]
                } else {
                    monkey.throws[1]
                };

                queue.push((next_monkey, worry));

                *inspected.get_mut(monkey_idx).expect(EXPECT_A_MONKEY) += 1;
            }

            monkey.items.clear();

            for (next_monkey, item) in queue {
                let monkey = monkeys.get_mut(next_monkey).expect(EXPECT_A_MONKEY);

                monkey.items.push_back(item);
            }
        }
    }

    inspected.sort_unstable();
    inspected.reverse();

    let monkey_business: usize = inspected.iter().take(2).product();

    Ok(Box::new(monkey_business))
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn puzzle_one() {
        let expected = "10605";

        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "2713310158";

        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}
