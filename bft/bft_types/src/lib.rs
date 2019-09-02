use std::io::{Result, Error, ErrorKind};
use std::path::Path;
use std::fs;
use std::fmt;
use std::string::ToString;

pub struct BrainfuckProgram {
    name: String,
    tokens: Vec<Token>
}

impl BrainfuckProgram {
    pub fn new(file_name: &dyn AsRef<str>, content: &dyn AsRef<str>) -> BrainfuckProgram {
        BrainfuckProgram {
            name: file_name.as_ref().to_string(),
            tokens: content.as_ref().chars()
                .filter_map(|c| Token::new(1, 1, c)) // todo this
                .collect()
        }
    }

    pub fn from_file(path: &dyn AsRef<Path>) -> Result<BrainfuckProgram> {
        Ok(Self::new(
            &path.as_ref().file_name()
                .ok_or_else(|| Error::new(
                    ErrorKind::Other, 
                    format!("Couldn't discern file name from \"{}\"", path.as_ref().display())
                ))?
                .to_string_lossy().to_string(),
            &fs::read_to_string(&path)?
        ))
    }

    pub fn get_instructions(&self) -> &[Token] {
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RawInstruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Output,
    Input,
    StartLoop,
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

#[derive(Debug, PartialEq)]
pub struct Token {
    line_number: u32,
    col_number: u32,
    instruction: RawInstruction,
}

impl Token {
    fn new(line_number: usize, col_number: usize, c: char) -> Option<Self> {
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
