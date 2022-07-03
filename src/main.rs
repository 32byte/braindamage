use std::{
    fs::{self, OpenOptions},
    io::{stdin, stdout, Write},
    process::Command
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

    #[clap(short, long, help = "Compiles your program to a binary", required = false)]
    compile: bool,

    #[clap(short, long, help = "Output file name of the binary", required = false)]
    output: Option<String>,
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

    if args.compile {
        let binary_name = args.output.unwrap_or("program.out".to_string());

        // compile and run the program
        let asm = braindamage::compile(&tokens);
        // write assembly to file
        fs::create_dir_all("./build").unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&format!("./build/{binary_name}.asm"))
            .unwrap();
        file.write_all(asm.as_bytes()).unwrap();
        drop(file);
        // compile the assembly using nasm
        Command::new("nasm")
            .args(["-f", "elf64"])
            .args(["-o", &format!("./build/{binary_name}.o")])
            .args([&format!("./build/{binary_name}.asm")])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        // link the program
        Command::new("ld")
            .args([&format!("./build/{binary_name}.o")])
            .args(["-o", &format!("{binary_name}")])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        match braindamage::interpret(&tokens, &mut stdin(), &mut stdout()) {
            Ok(_) => (),
            Err(RuntimeError::IOError) => panic!("An IO Error occured!"),
            Err(RuntimeError::PtrOutOfBounds(ptr)) => {
                panic!("Trying to read/write out of bounds at pos {}!", ptr)
            }
            // Should never be out of input when using stdin as input
            Err(RuntimeError::OutOfInput) => unreachable!(),
        }
    }
}
