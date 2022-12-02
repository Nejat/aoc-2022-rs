use std::fs::File;
use std::io;
use std::path::PathBuf;

mod dec_01;
mod dec_02_one;
mod dec_02_two;

// a collection of puzzles
type Puzzles<'a> = Vec<(&'a str, &'a str, Box<dyn Fn(File) -> io::Result<Box<dyn ToString>>>)>;

fn main() -> io::Result<()> {
    let puzzles: Puzzles = vec![
        ("dec 1st puzzle one", "2022-12-01.txt", Box::new(dec_01::puzzle_one)),
        ("dec 1st puzzle two", "2022-12-01.txt", Box::new(dec_01::puzzle_two)),
        ("dec 2nd puzzle one", "2022-12-02.txt", Box::new(dec_02_one::puzzle_one)),
        ("dec 2nd puzzle two", "2022-12-02.txt", Box::new(dec_02_two::puzzle_two)),
    ];

    for (label, input_file, puzzle) in puzzles {
        let mut puzzle_input = PathBuf::from("puzzle_input");

        puzzle_input.push(input_file);

        let input = File::open(puzzle_input)?;

        println!("{}: {:?}", label, puzzle(input)?.to_string());
    }

    Ok(())
}
