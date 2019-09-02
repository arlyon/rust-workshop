use bft_types::BrainfuckProgram;
use bft_interp::BrainfuckInterpreter;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = env::args().nth(1).ok_or("No file name provided!")?;
    let program = BrainfuckProgram::from_file(&file_name)?;
    let interpreter: BrainfuckInterpreter<u8> = BrainfuckInterpreter::new(30_000, true);
    Ok(interpreter.interpret(&program))
}
