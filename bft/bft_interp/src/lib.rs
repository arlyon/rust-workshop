//! Contains the logic for the brainfuck interpreter.

use bft_types::{BrainfuckProgram, RawInstruction};
use std::fmt::Display;

/// Encapsulates the local state of a Brainfuck interpreter.
/// Once created, the interpreter accepts [BrainfuckProgram].
pub struct BrainfuckInterpreter<T> {
    tape: Vec<T>,
    tape_index: usize,
    expandable: bool,
}

pub trait IncDec {
    fn increment(&self) -> Self;
    fn decrement(&self) -> Self;
}

impl IncDec for u8 {
    fn increment(&self) -> Self { self + 1 }
    fn decrement(&self) -> Self { self + 1 }
}


impl<T> BrainfuckInterpreter<T>
where T: std::ops::AddAssign, T: std::ops::SubAssign, T: Default, T: IncDec {

    /// Creates a new [BrainfuckInterpreter] with an initial tape size.
    /// By setting `expandable` to `true`, the interpreter will automatically
    /// expand the tape size as needed if the pointer overflows.
    /// 
    /// **note:** Setting `tape_size` to 0 results in the interpreter using
    ///           the default size of 30,000.
    /// 
    /// # Example Usage
    /// 
    /// ```
    /// let interp = BrainfuckInterpreter<u8>::new(50, false);
    /// ```
    pub fn new(tape_size: usize, expandable: bool) -> BrainfuckInterpreter<T> {
        let tape_size = if tape_size == 0 {30_000} else {tape_size};
        BrainfuckInterpreter {
            tape: Vec::with_capacity(tape_size),
            tape_index: 0,
            expandable,
        }
    }

    /// Interprets the given [BrainfuckProgram] on the [BrainfuckInterpreter].
    /// 
    /// **note:** This does not reset the tape state. If you need fresh state, 
    ///           use [BrainfuckInterpreter::reset]. This is so that multiple
    ///           programs could be chained together.
    pub fn interpret(&mut self, program: &BrainfuckProgram) {
        println!("Running program {}\n\n", program.name);
        println!("{}", program.get_tokens().iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(""))
    
        let instruction_counter = 0;
        let tokens = program.get_tokens();

        while instruction_counter < tokens.len() {
            match tokens[instruction_counter].instruction {
                RawInstruction::IncrementPointer => self.tape_index += 1,
                RawInstruction::DecrementPointer => self.tape_index -= 1,
                RawInstruction::Increment => {self.tape[self.tape_index].increment();},
                RawInstruction::Decrement => {self.tape[self.tape_index].decrement();},
                RawInstruction::Input => (),
                RawInstruction::Output => (),
                RawInstruction::StartLoop => (),
                RawInstruction::EndLoop => (),
            }
        }
    }

    /// Resets the state (memory) of the interpreter.
    pub fn reset(&mut self) {
        self.tape_index = 0;
        self.tape = Vec::with_capacity(self.tape.capacity())
    }
}

#[cfg(test)]
mod tests {

}
