use braindamage::{self, RuntimeError, Token};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    process::{Command, Output, Stdio},
};

fn compile_program(tokens: &[Token], name: &str, stdin: &mut impl Read) -> Output {
    // compile and run the program
    let asm = braindamage::compile(tokens);
    // write assembly to file
    fs::create_dir_all("./build/tests").unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("./build/tests/{name}.asm"))
        .unwrap();
    file.write_all(asm.as_bytes()).unwrap();
    drop(file);
    // compile the assembly using nasm
    let nasm_output = Command::new("nasm")
        .args(["-f", "elf64"])
        .args(["-o", &format!("./build/tests/{name}.o")])
        .args([&format!("./build/tests/{name}.asm")])
        .output()
        .unwrap();
    assert!(nasm_output.stderr.is_empty());
    // link the program
    let ld_output = Command::new("ld")
        .args([&format!("./build/tests/{name}.o")])
        .args(["-o", &format!("./build/tests/{name}")])
        .output()
        .unwrap();
    assert!(ld_output.stderr.is_empty());
    // run the program
    let mut child = Command::new(format!("./build/tests/{name}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // need to create temporary buffer (i think?)
    let buf: Result<Vec<u8>, _> = stdin.bytes().collect();
    // forward input to the program
    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(&buf.unwrap()).unwrap();
    }
    // close child stdin
    child.wait_with_output().unwrap()
}

#[test]
fn test_braindamage() {
    let program = include_str!("../examples/braindamage.b");
    let tokens = braindamage::parse(program).unwrap();

    // interpret the program
    let mut interpretted_output = Vec::new();

    let _ = braindamage::interpret(&tokens, &mut "".as_bytes(), &mut interpretted_output).unwrap();
    let compiled_output = compile_program(&tokens, "braindamage", &mut "".as_bytes());

    assert_eq!(interpretted_output, compiled_output.stdout);
}

#[test]
fn test_hello_world() {
    let program = include_str!("../examples/hello-world.b");
    let tokens = braindamage::parse(program).unwrap();

    // interpret the program
    let mut interpretted_output = Vec::new();

    let _ = braindamage::interpret(&tokens, &mut "".as_bytes(), &mut interpretted_output).unwrap();
    let compiled_output = compile_program(&tokens, "hello-world", &mut "".as_bytes());

    assert_eq!(interpretted_output, compiled_output.stdout);
}

#[test]
fn test_rot13() {
    let program = include_str!("../examples/rot13.b");
    let tokens = braindamage::parse(program).unwrap();

    // interpret the program
    let mut interpretted_output = Vec::new();

    let input = "Hello, World";
    let err = braindamage::interpret(&tokens, &mut input.as_bytes(), &mut interpretted_output)
        .unwrap_err();
    assert_eq!(err, RuntimeError::OutOfInput);

    let compiled_output = compile_program(&tokens, "rot13", &mut input.as_bytes());
    assert_eq!(interpretted_output, compiled_output.stdout);
}
