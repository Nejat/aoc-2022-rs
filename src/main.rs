use std::fs::File;
use std::io;
use std::path::PathBuf;

mod dec_first;

// a collection of puzzles
type Puzzles<'a> = Vec<(&'a str, &'a str, Box<dyn Fn(File) -> io::Result<Box<dyn ToString>>>)>;

fn main() -> io::Result<()> {
    let puzzles: Puzzles = vec![
        ("dec 1st puzzle one", "2022-12-01.txt", Box::new(dec_first::puzzle_one)),
        ("dec 1st puzzle two", "2022-12-01.txt", Box::new(dec_first::puzzle_two)),
    ];

    for (label, input_file, puzzle) in puzzles {
        let mut puzzle_input = PathBuf::from("puzzle_input");

        puzzle_input.push(input_file);

        let input = File::open(puzzle_input)?;

        println!("{}: {:?}", label, puzzle(input)?.to_string());
    }

    Ok(())
}
