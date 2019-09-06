//! Contains supporting types and datastructures
//! for the brainfuck interpreter.

use std::io::{Result, Error, ErrorKind, BufReader, BufRead};
use std::path::Path;
use std::fs;
use std::fmt;
use std::string::ToString;

/// A brainfuck program, ie. a list of valid brainfuck instructions.
pub struct BrainfuckProgram {
    pub name: String,
    tokens: Vec<Token>
}

impl BrainfuckProgram {

    /// Creates a new [BrainfuckProgram] with the given file name and content.
    pub fn new(file_name: &dyn AsRef<str>, tokens: Vec<Token>) -> BrainfuckProgram {
        BrainfuckProgram {
            name: file_name.as_ref().to_string(),
            tokens,
        }
    }

    /// Creates a new [BrainfuckProgram] with the given file name and content.
    pub fn from_string(file_name: &dyn AsRef<str>, content: &dyn AsRef<str>) -> BrainfuckProgram {
        BrainfuckProgram {
            name: file_name.as_ref().to_string(),
            tokens: content.as_ref().chars()
                .filter_map(|c| Token::new(1, 1, c)) // todo this
                .collect()
        }
    }

    /// Parses a new [BrainfuckProgram] at a given path relative to the current directory.
    pub fn from_file(path: &dyn AsRef<Path>) -> Result<BrainfuckProgram> {

        let name = path.as_ref().file_name()
                .ok_or_else(|| Error::new(
                    ErrorKind::Other, 
                    format!("Couldn't discern file name from \"{}\"", path.as_ref().display())
                ))?
                .to_string_lossy().to_string();

        let text = BufReader::new(fs::File::open(path)?);
        let tokens: Result<_> = text.lines()
            .enumerate()
            .map(|(row, line)| Ok(
                line?.chars().filter_map(|c| Token::new(1,1,c))
            ))
            .collect();

        Ok(Self::new(
            &name,
            tokens?,
        ))
    }

    pub fn get_tokens(&self) -> &[Token] {
        self.tokens.as_slice()
    }
}

impl std::string::ToString for BrainfuckProgram {
    fn to_string(&self) -> String {
        self.tokens.iter().map(|t| format!("{}", t)).collect()
    }
}

static INSTRUCTIONS: &[(char, RawInstruction)] = &[
    ('>', RawInstruction::IncrementPointer),
    ('<', RawInstruction::DecrementPointer),
    ('+', RawInstruction::Increment),
    ('-', RawInstruction::Decrement),
    ('.', RawInstruction::Output),
    (',', RawInstruction::Input),
    ('[', RawInstruction::StartLoop),
    (']', RawInstruction::EndLoop)
];

/// Enumerates all the instructions in valid brainfuck code.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RawInstruction {
    /// Increments the currently-pointed-at index by one.
    IncrementPointer,

    /// Decrements the currently-pointed-at index by one.
    DecrementPointer,

    /// Increments the byte at the currently-pointed-at index by one.
    Increment,

    /// Decrements the byte at the currently-pointed-at index by one.
    Decrement,

    /// Prints the byte at the currently-pointed-at index.
    Output,

    /// Reads a byte from stdin into the currently-pointed-at index.
    Input,

    /// If the byte at the currently-pointed-at index is 0, jump the instruction 
    /// pointer to the instruction after the next [RawInstruction::EndLoop].
    StartLoop,

    /// If the byte at the currently-pointed-at index is 0, jump the instruction 
    /// pointer to the instruction after the previous [RawInstruction::StartLoop].
    EndLoop,
}

impl RawInstruction {
    fn from_char(c1: char) -> Option<Self> {
        INSTRUCTIONS.iter().find(|(c2, _)| c1.eq(c2)).map(|(_, ri)| *ri)
    }

    fn to_char(self) -> char {
        INSTRUCTIONS.iter().find(|(_, ri)| self.eq(ri)).map(|(c2, _)| *c2).unwrap()
    }
}

impl std::fmt::Display for RawInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Represents a token in a brainfuck program, which consists of
/// a line number, column number, and [RawInstruction].
#[derive(Debug, PartialEq)]
pub struct Token {
    line_number: u32,
    col_number: u32,
    pub instruction: RawInstruction,
}

impl Token {
    /// Attempts to create a new [Token], returning [Some] if the 
    /// [char] is a valid [RawInstruction] and [None] otherwise.
    pub fn new(line_number: usize, col_number: usize, c: char) -> Option<Self> {
        Some(Self {
            line_number: line_number as u32,
            col_number: col_number as u32,
            instruction: RawInstruction::from_char(c)?,
        })
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.instruction.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Token, RawInstruction, BrainfuckProgram};

    #[test]
    fn create_invalid_token() {
        assert_eq!(Token::new(1, 1, 'f'), None);
    }

    #[test]
    fn create_valid_token() {
        assert_eq!(
            Token::new(1,1,'<').unwrap(), 
            Token{
                line_number: 1, 
                col_number: 1, 
                instruction: RawInstruction::from_char('<').unwrap()
            }
        );
    }

    #[test]
    fn instr_char_convert() {
        assert_eq!(
            '<',
            RawInstruction::from_char('<').unwrap().to_char()
        )
    }

    macro_rules! parse_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (file, content, expected) = $value;
                assert_eq!(
                    BrainfuckProgram::new(file, content).to_string(),
                    expected
                )
            }
        )*
        }
    }

    parse_tests! {
        parse_comment: (&"./virtual", &"<><>comment<><>", "<><><><>"),
        parse_all: (&"./virtual", &"<>.,+-[]", "<>.,+-[]"),
        parse_none: (&"./virtual", &"", ""),
    }
}
