use std::io;
use std::io::Read;
use std::ops::Deref;
use std::str::FromStr;

use crate::utils::{CleansedLines, io_error, sum_everything};

/// iterator of cpu instructions
struct Instructions<R> {
    lines: CleansedLines<R>,
}

impl<R> Instructions<R>
    where R: Read
{
    fn new(input: R) -> Self {
        Self {
            lines: CleansedLines::new(input),
        }
    }
}

impl<R> Iterator for Instructions<R>
    where R: Read
{
    type Item = io::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        // get next instruction
        let next = self.lines.next()?.ok()?;

        // parse instruction
        Some(next.parse::<Instruction>().
            map_err(|err| io_error(&err.to_string())))
    }
}

// simple cpu with two instructions
struct CPU {
    cycles: usize,
    current_op: Option<Instruction>,
    register_x: isize,
}

impl CPU {
    const fn new() -> Self {
        Self {
            cycles: 1,
            current_op: None,
            register_x: 1,
        }
    }

    fn run_instructions<R>(&mut self, instructions: Instructions<R>) -> Cycles<R>
        where R: Read
    {
        Cycles::new(self, instructions)
    }
}

/// iterator of `cpu` `instruction` cycles
struct Cycles<'a, R> {
    cpu: &'a mut CPU,
    cycles: usize,
    instructions: Instructions<R>,
}

impl<'a, R> Cycles<'a, R>
    where R: Read
{
    fn new(cpu: &'a mut CPU, instructions: Instructions<R>) -> Self {
        Self {
            cpu,
            cycles: 0,
            instructions,
        }
    }
}

impl<'a, R> Iterator for Cycles<'a, R>
    where R: Read
{
    type Item = io::Result<(usize, isize)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // check if cpu is executing an instruction
            if self.cpu.current_op.is_none() {
                // get next instruction
                let next = self.instructions.next()?.ok()?;

                // setup cycles and cpu
                self.cycles = *next;
                self.cpu.current_op = Some(next);
            } else {
                // current state of cpu
                let current_state = (self.cpu.cycles, self.cpu.register_x);

                // update next cpu cycle
                self.cpu.cycles += 1;
                // update current instruction cycle
                self.cycles -= 1;

                // check if current instruction is complete
                if self.cycles == 0 {
                    // if current instruction is addx
                    if let Some(Instruction::AddX(value)) = self.cpu.current_op {
                        // update register with addx operand
                        self.cpu.register_x += value;
                    }

                    // mark current operation complete
                    self.cpu.current_op = None;
                }

                // return current state
                return Some(Ok(current_state));
            }
        }
    }
}

/// `cpu` instructions
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

/// a reference of an `instruction` represents it's `cpu` cycles
impl Deref for Instruction {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Noop => &1,
            Self::AddX(_) => &2
        }
    }
}

/// parse a `cpu` `instruction` from a `str`
impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source.trim();

        if source.eq_ignore_ascii_case("noop") {
            Ok(Self::Noop)
        } else if let Some((op, value)) = source.split_once(' ') {
            if op.trim().eq_ignore_ascii_case("addx") {
                let value = value.parse::<isize>()
                    .map_err(|err| io_error(&format!("'{value}' is not a valid value for `addx` instruction; {err}")))?;

                Ok(Self::AddX(value))
            } else {
                Err(io_error(&format!("'{op}' is not a valid instruction")))
            }
        } else {
            Err(io_error(&format!("'{source}' is not a valid instruction")))
        }
    }
}

/// iterator of signals processed from `cpu`
struct SignalProcessor<'a, R> {
    cycles: Cycles<'a, R>,
    trigger_freq: usize,
    trigger_offset: usize,
}

impl<'a, R> SignalProcessor<'a, R>
    where R: Read
{
    const fn new(cycles: Cycles<'a, R>, trigger_offset: usize, trigger_freq: usize) -> Self {
        Self { cycles, trigger_freq, trigger_offset }
    }
}

impl<'a, R> Iterator for SignalProcessor<'a, R>
    where R: Read,
{
    type Item = io::Result<isize>;

    #[allow(clippy::cast_possible_wrap)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (cycle, register_x) = self.cycles.next()?.ok()?;

            // process signal based on input parameters
            if cycle >= self.trigger_offset && 
                (cycle - self.trigger_offset) % self.trigger_freq == 0 
            {
                return Some(Ok(cycle as isize *  register_x));
            }
        }
    }
}

/// `CRT` that displays frames from a signal processor
struct CRT {
    pixels: Vec<u8>,
    size: Size,
}

impl CRT {
    fn new(size: Size) -> Self {
        // draw an empty frame of pixels
        let pixels = (0..size.height)
            .map(|_| ".".repeat(size.width))
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes();

        Self {
            pixels,
            size,
        }
    }

    // updates the `crt` frame from input signals of `cpu`'s `cycle` and `register_x`
    #[allow(clippy::cast_possible_wrap)]
    fn refresh_frame<ERR>(
        &mut self,
        signals: impl Iterator<Item=Result<(usize, isize), ERR>>,
    ) -> Result<(), ERR> {
        for signal in signals {
            let (mut cycle, register_x) = signal?;

            cycle -= 1; // crt frame is zero based

            let y = cycle / self.size.width;
            let x = cycle % self.size.width;
            let sprite = register_x - 1..=register_x + 1;
            let pixel = cycle + y; // pixel adjusted for display line feeds
            let pattern = if sprite.contains(&(x as isize)) { b'#' } else { b'.' };

            self.pixels[pixel] = pattern;
        }

        Ok(())
    }
}

impl ToString for CRT {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.pixels).to_string()
    }
}

/// represents a size value of height and width
#[derive(Copy, Clone, Debug, Default)]
struct Size {
    height: usize,
    width: usize,
}

impl Size {
    const fn new(width: usize, height: usize) -> Self {
        Self { height, width }
    }
}

/// total of signal processor of a `cpu`'s `cycle` and `register_x` output
pub fn puzzle_one<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const TRIGGER_FREQUENCY: usize = 40;
    const TRIGGER_OFFSET: usize = 20;

    let mut cpu = CPU::new();
    let instructions = Instructions::new(input);
    let cycles = cpu.run_instructions(instructions);
    let signals = SignalProcessor::new(cycles, TRIGGER_OFFSET, TRIGGER_FREQUENCY);

    Ok(sum_everything(signals).map(Box::new)?)
}

/// decode a `crt` frame from a `cpu`'s `cycle` and `register_x` output
pub fn puzzle_two<R>(input: R) -> io::Result<Box<dyn ToString>>
    where R: Read
{
    const CRT_WIDTH: usize = 40;
    const CRT_HEIGHT: usize = 6;

    let mut cpu = CPU::new();
    let instructions = Instructions::new(input);
    let cycles = cpu.run_instructions(instructions);
    let mut crt = CRT::new(Size::new(CRT_WIDTH, CRT_HEIGHT));

    crt.refresh_frame(cycles)?;

    // println!("{}", crt.to_string());

    // actual input interprets to "RKPJBPLA"
    Ok(Box::new(crt.to_string().replace('\n', "")))
}

#[cfg(test)]
mod tests {
    use crate::EXPECTED_PUZZLE_SOLUTION;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    #[allow(clippy::too_many_lines)]
    fn puzzle_one() {
        let expected = "13140";

        let actual = super::puzzle_one(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn puzzle_two() {
        let expected = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";

        let actual = super::puzzle_two(INPUT.as_bytes())
            .expect(EXPECTED_PUZZLE_SOLUTION)
            .to_string();

        assert_eq!(actual, expected);
    }
}
