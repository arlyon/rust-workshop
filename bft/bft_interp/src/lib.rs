use bft_types::{BrainfuckProgram, RawInstruction};
use std::fmt::Display;

pub struct BrainfuckInterpreter<T> {
    tape: Vec<T>,
    expandable: bool,
}

impl<T> BrainfuckInterpreter<T> {
    pub fn new(tape_size: usize, expandable: bool) -> BrainfuckInterpreter<T> {
        let tape_size = if tape_size == 0 {30_000} else {tape_size};
        BrainfuckInterpreter {
            tape: Vec::with_capacity(tape_size),
            expandable,
        }
    }

    pub fn interpret(&self, program: &BrainfuckProgram) {
        println!("{}", program.get_instructions().iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(""))
    }
}

#[cfg(test)]
mod tests {

}
