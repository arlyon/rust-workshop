/*
Problem 2 - In for the long haul
--------------------------------

You now have an input file with the following content:

    ---8<---
    This is your input file

    Anything which is not one of the important characters will be ignored
    by the program

    The program is as follows:

       +[-
         [<<
          [+
           [--->]
          -[<<<]
         ]
        ]>>>-
       ]

    I'm guessing that you know where we're going with all this

       >-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.

    But perhaps you're as lost as a little lamb?
    ---8<---

As you can no doubt tell, this is a program in the [Brainfuck][] language.

For now, we're not going to be writing anything as complex as a Brainfuck
interpreter, though we will be building one in the future so bear that
in mind as you write today's homework answer.

[Brainfuck]: https://en.wikipedia.org/wiki/Brainfuck

For today, please write a program which, when run as:

    $ cargo run -- inputfile

will read `inputfile`, reduce it to the list of valid Brainfuck characters
(i.e. strip out all of the comment characters) and then print out the
program.  There are eight valid command characters, and they're listed in
the Wikipedia article.

You can do this entirely with things we've already considered, by means of
iterators you can find on `BufRead` and on `str`, and with judicious use of
`match` and `print!()/println!()`

However, for bonus points, make use of `Vec<char>`, `Vec<u8>` or `String` and
learn how to use `collect()` from the `Iterator` trait.

Note: When I test your homework, I'll use different input, so no cheating.

*/

use std::env;
use std::fs;
use std::error;
use std::fmt;

fn main() -> Result<(), Box<dyn error::Error>> {
    let file_name = env::args().nth(1).ok_or("No file name provided!")?;

    let content = fs::read_to_string(&file_name)?;

    let filtered_content = content.chars()
        .filter(|c1| "<>+-.,[]".chars().any(|c2| c2.eq(c1)))
        .collect::<String>();

    Ok(println!("{}", filtered_content))
}