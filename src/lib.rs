use std::io::{Read, Write};

pub const BUF_SIZE: usize = 30_000;
pub const EOF: u8 = 0;
pub const NEW_LINE: u8 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // >
    IncPtr,
    // <
    DecPtr,
    // +
    IncByte,
    // -
    DecByte,
    // .
    WriteByte,
    // ,
    ReadByte,
    // [
    LoopStart(usize),
    // ]
    LoopEnd(usize),
}

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::IncPtr),
            '<' => Some(Self::DecPtr),
            '+' => Some(Self::IncByte),
            '-' => Some(Self::DecByte),
            '.' => Some(Self::WriteByte),
            ',' => Some(Self::ReadByte),
            '[' => Some(Self::LoopStart(0)),
            ']' => Some(Self::LoopEnd(0)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    // Unexpected ']' at position (no previous '[')
    UnexpectedClosingBracket(usize),
    // (Multiple) unclosed '[', usize -> amount
    ExpectedClosingBracket(usize),
}

pub fn parse(str: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens: Vec<Token> = Vec::new();
    // stack for parsing brackets
    let mut stack: Vec<usize> = Vec::new();

    for (_, char) in str.char_indices() {
        let token = Token::from_char(char);

        match token {
            // '[' -> push index onto bracket stack
            Some(Token::LoopStart(p)) => {
                // Note: don't use index since it also counts ignored characters
                stack.push(tokens.len());
                tokens.push(Token::LoopStart(p));
            }
            // ']' -> retreive last opening bracket index from the stack
            //        update the Loop start and Loop end
            Some(Token::LoopEnd(_)) => {
                let opening = if let Some(pos) = stack.pop() {
                    pos
                } else {
                    return Err(ParseError::UnexpectedClosingBracket(tokens.len()));
                };

                tokens[opening] = Token::LoopStart(tokens.len());
                tokens.push(Token::LoopEnd(opening));
            }
            // Any other token, just add to the tokens array
            Some(token) => tokens.push(token),
            // Invalid tokens return None and are treated as comments
            None => (),
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::ExpectedClosingBracket(stack.len()));
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    PtrOutOfBounds(i32),
    OutOfInput,
    IOError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Buffer(pub [u8; BUF_SIZE]);

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self([0; BUF_SIZE])
    }

    fn bounds_check(&self, index: i32) -> Option<usize> {
        if index < 0 || index >= BUF_SIZE as i32 {
            return None;
        }

        Some(index as usize)
    }

    pub fn get_checked(&self, index: i32) -> Option<&u8> {
        let index = self.bounds_check(index)?;

        Some(&self.0[index])
    }

    pub fn get_mut_checked(&mut self, index: i32) -> Option<&mut u8> {
        let index = self.bounds_check(index)?;

        Some(&mut self.0[index])
    }
}

pub fn interpret(
    tokens: &Vec<Token>,
    input: &mut impl Read,
    output: &mut impl Write,
) -> Result<Buffer, RuntimeError> {
    // Buffer to hold all the data
    let mut buf: Buffer = Buffer::new();
    // Buffer pointer, i32 for easier calculations (Note: a bounds check will be performed)
    let mut ptr: i32 = 0;
    // Instruction pointer
    let mut pos: usize = 0;

    while pos < tokens.len() {
        let token = &tokens[pos];

        match token {
            Token::IncPtr => (ptr += 1),
            Token::DecPtr => (ptr -= 1),
            Token::IncByte => {
                if let Some(byte) = buf.get_mut_checked(ptr) {
                    // Add wrapping since that is the expected behaviour
                    *byte = byte.wrapping_add(1);
                } else {
                    return Err(RuntimeError::PtrOutOfBounds(ptr));
                }
            }
            Token::DecByte => {
                if let Some(byte) = buf.get_mut_checked(ptr) {
                    // Sub wrapping since that is the expected behaviour
                    *byte = byte.wrapping_sub(1);
                } else {
                    return Err(RuntimeError::PtrOutOfBounds(ptr));
                }
            }
            Token::WriteByte => {
                if let Some(byte) = buf.get_checked(ptr) {
                    write!(output, "{}", *byte as char).map_err(|_| RuntimeError::IOError)?;
                    output.flush().map_err(|_| RuntimeError::IOError)?;
                } else {
                    return Err(RuntimeError::PtrOutOfBounds(ptr));
                }
            }
            Token::ReadByte => {
                if let Some(char) = input.bytes().next() {
                    if let Some(byte) = buf.get_mut_checked(ptr) {
                        *byte = char.map_err(|_| RuntimeError::IOError)?;
                    } else {
                        return Err(RuntimeError::PtrOutOfBounds(ptr));
                    }
                } else {
                    return Err(RuntimeError::OutOfInput);
                }
            }
            Token::LoopStart(end) => {
                if let Some(byte) = buf.get_checked(ptr) {
                    // If "!byte" skip the loop
                    if *byte == 0 {
                        pos = *end;
                    }
                } else {
                    return Err(RuntimeError::PtrOutOfBounds(ptr));
                }
            }
            Token::LoopEnd(start) => {
                // Subtract 1 because 1 will be added later
                // (=> next iteration pos = start)
                pos = *start - 1;
            }
        }

        pos += 1;
    }

    Ok(buf)
}

pub fn compile(tokens: &[Token]) -> String {
    let mut output = Vec::<String>::new();

    // NASM
    // Boilerplate for the assembly
    let boilerplate: Vec<String> = include_str!("./boilerplate.asm")
        .lines()
        .map(|s| s.to_owned())
        .collect();
    output.extend(boilerplate);

    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::IncPtr => output.push("\tadd qword r12, 1".to_owned()),
            Token::DecPtr => output.push("\tsub qword r12, 1".to_owned()),
            Token::IncByte => output.push("\tadd byte [r12], 1".to_owned()),
            Token::DecByte => output.push("\tsub byte [r12], 1".to_owned()),
            Token::WriteByte => output.push("\tcall writec".to_owned()),
            Token::ReadByte => output.push("\tcall readc".to_owned()),
            Token::LoopStart(end) => {
                // jump to end if r12 is 0
                output.push("\tcmp byte [r12], 0".to_owned());
                output.push(format!("\tje LOOP_END_{}", end));

                output.push(format!("LOOP_START_{}:", idx));
            }
            Token::LoopEnd(start) => {
                output.push("\tcmp byte [r12], 0".to_owned());
                output.push(format!("\tjne LOOP_START_{}", start));
                output.push(format!("LOOP_END_{}:", idx));
            }
        };
    }
    output.push("\tcall exit".to_owned());

    output.join("\n")
}
