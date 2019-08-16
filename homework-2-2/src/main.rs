/*
Part 2 - Another step toward an interpreter
===========================================

We're going to begin developing a BF interpreter soon.  As such let's do some
work toward representing the program usefully.

Using the same input as you had from the first week, we're going to build an
intermediate representation of the program.

First up, please write an `enum` for BF instructions.  Remember that there are
eight possible instruction characters.  These are probably *raw* instructions
since we'll later want to do something clever for the loops, so name your enum
appropriately.

Implement an associated function `RawInstruction::from_char` which returns
an `Option<RawInstruction>` which is `None` if the input character isn't a valid
raw BF instruction.

If you'd prefer, you can implement the above as TryFrom<Error=()>, though it
probably makes more sense to do it as I suggest, returning an Option<> instead.

Next, I would like a struct to represent each input instruction, using the
RawInstruction enumeration from above, and also a pair of `usize` values to
represent the line number and character column of the instruction.

Please write a function like in part 1 which takes a filename and returns a
Result<Vec<StructFromAbove>, Box<dyn std::error::Error>>.

Use a debugging `println!` to ensure the parsed data is useful.

What I'd like as the final output of this part is something akin to:

    [inputfile:8:4] Increment
    [inputfile:8:5] StartLoop
    [inputfile:8:6] Decrement
    ...

This might require you to add a `Display` implementation for your `RawInstruction`
and appropriate methods on your struct to get hold of the line and column numbers.
Also make sure you give human numbering for lines and columns - humans count from
one, not zero.

As you implement this last part you may come across an error akin to:
"cannot move out of borrowed context" when implementing your accessor method
for the `RawInstruction`.  If you do, you might find the `Clone` and `Copy`
traits useful to derive on your nice simple enumeration.
*/

use std::env;
use std::fs;

#[derive(Debug)]
enum RawInstruction {
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
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::IncrementPointer),
            '<' => Some(Self::DecrementPointer),
            '+' => Some(Self::Increment),
            '-' => Some(Self::Decrement),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::StartLoop),
            ']' => Some(Self::EndLoop),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Token {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = env::args().nth(1).ok_or("No file name provided!")?;
    let tokens = parse_tokens_from_file(&file_name)?;

    println!("{:?}", tokens);
    if tokens.is_empty() {
        return Ok(());
    }

    let line_number_digits = calculate_digits(tokens.last().unwrap().line_number);
    let col_number_digits = calculate_digits(tokens.iter().map(|t| t.col_number).max().unwrap());

    for token in tokens {
        println!(
            "[{}:{:0line_width$}:{:0col_width$}] {:?}",
            file_name,
            token.line_number,
            token.col_number,
            token.instruction,
            line_width = line_number_digits,
            col_width = col_number_digits
        );
    }

    Ok(())
}

fn calculate_digits(n: u32) -> usize {
    let mut digits = 0;
    let mut n = n;
    while n != 0 {
        digits += 1;
        n /= 10;
    }
    digits as usize
}

fn parse_tokens_from_file(file_name: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&file_name)?;
    Ok(content
        .lines()
        .enumerate()
        .flat_map(|(line, text)| {
            text.chars()
                .enumerate()
                .filter_map(move |(col, chr)| Token::new(line + 1, col + 1, chr))
        })
        .collect())
}
