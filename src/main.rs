use std::{
    fs,
    io::{stdin, stdout},
};

use braindamage::{ParseError, RuntimeError};
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(
            ArgGroup::new("input")
                .required(true)
                .args(&["file", "program"]),
        ))]
struct Args {
    #[clap(short, long, help = "Path to the program file")]
    file: Option<String>,

    #[clap(short, long, help = "Interprets the argument as a program")]
    program: Option<String>,
}

fn main() {
    let args = Args::parse();

    let content = if let Some(path) = args.file {
        fs::read_to_string(&path).expect(&format!("Couldn't read {}", path))
    } else {
        // Safe to unwrap since clap makes sure that either `file` or `program` is some.
        args.program.unwrap()
    };

    let tokens = match braindamage::parse(&content) {
        Ok(tokens) => tokens,
        Err(ParseError::UnexpectedClosingBracket(pos)) => {
            panic!("Unexpected closing bracket at {}", pos)
        }
        Err(ParseError::ExpectedClosingBracket(num)) => {
            panic!("Expected {} more closing backets", num)
        }
    };

    match braindamage::interpret(&tokens, &mut stdin(), &mut stdout()) {
        Ok(_) => (),
        Err(RuntimeError::IOError) => panic!("An IO Error occured!"),
        Err(RuntimeError::PtrOutOfBounds(ptr)) => {
            panic!("Trying to read/write out of bounds at pos {}!", ptr)
        }
    }
}
