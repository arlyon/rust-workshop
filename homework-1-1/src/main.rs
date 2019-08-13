/*
Problem 1: Adding up numbers
----------------------------

You have an input file whose content is as follows (ignoring the obvious
start/end markers):

    ---8<---
    14
    9
    7
    1
    -5
    0
    -2
    9
    1
    -100
    75623
    -65536
    -100
    5
    2
    100
    46
    ---8<---

Your goal is to write a program which when run as follows:

    $ cargo run -- inputfile

will read `inputfile`, break it into lines, parse each line as a number of some
useful type, sum all the numbers, and print the sum to stdout.

If any of the lines does not parse as a suitable number, please just return
the error out of your main function, using the techniques we learned in the
10 green bottles example.

Bonus points if you make interesting use of `Iterator` methods.

Note: When I test your homework, I won't use the above input, so no cheating.
*/

/*
QUESTIONS:

I'd be keen to learn if there is a nicer way to express
"map and then sum into a Result, collecting all errors from the map"
Result has a sum method, but it only return the first error...
*/

use std::env;
use std::fs;
use std::fmt;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file_name = env::args().nth(1).ok_or("No file name provided!")?;

    let file_content = fs::read_to_string(&file_name)?;

    // Ok(line number, Result) and Err(line_number, Result)
    let (oks, errors): (Vec<_>, Vec<_>) = file_content.split('\n')
        .map(str::parse::<i32>)
        .enumerate()
        .partition(|(_, f)| f.is_ok());

    let numbers = oks.iter().map(|(_, f)| f.as_ref().unwrap());
    let mut errors = errors.iter().map(|(i, f)| (i+1, f.as_ref().unwrap_err())).peekable();

    if errors.peek().is_none() {
        Ok(println!("{}", numbers.sum::<i32>()))
    } else {
        Err(format!("Errors found during parse: {}", errors
            .map(|(i, e)| format!("on line {} ({:})", i, e.to_string()))
            .collect::<Vec<_>>()
            .join(", ")
        ).into())
    }
}