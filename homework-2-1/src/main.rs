/*
Session 2 homework
==================

As last week, the first part will be practice for what we've looked at this week
while part two will be building up the work toward a BF interpreter.

This homework block is much larger and more complex than the first set of homework
so please don't wait to get started.  The sooner you try, the sooner you can ask
should you encounter any issues.  Again, homework is due in on the day before the
next session of the workshop, as zip or tarball of source code, or a link to a
git repository for me to read.  Again, I'll be checking your code against inputs
not listed here in the input document; and again the cut marks `---8<---` are not
to be considered part of your valid input and will not form part of any test vectors.

Part 1 - Practice for structs and enumerations
----------------------------------------------

For this homework task you do not have complete control over the data structures
since I want you to practice certain things so I'm guiding the code design
a little.  It'll not be glorious, but you will get practice.  As such, marking
will be more strict for this question.  I want you to demonstrate that you truly
understand each thing I'm having you do.

This is a big part 1 but you'll have learned a lot ready for part 2.

You will have input in the form:

    ---8<---
    Harry:9
    Susan:4
    Gregory:3
    Ulrika:10
    Daniel Silverstone:9
    Fred Jones:10
    Harry:4
    Susan
    Gregory:7
    Ulrika:9
    Daniel Silverstone:7
    Fred Jones:7
    Harry:6
    Susan:6
    Gregory:9
    Ulrika:10
    Daniel Silverstone
    Fred Jones:4
    Harry:7
    Susan:9
    Gregory:8
    Ulrika:7
    Daniel Silverstone:10
    Fred Jones
    ---8<---

I would like you to define a enum to represent a line.  It should have
two variants.  One for the form "name:number" and one for the form "name".

I want you to implement `std::convert::TryFrom` for your enumeration, converting
from either `&str`, `&String` or, if you're feeling exuberant, from `AsRef<str>`.

You should be able to parse the "name:number" form into a string and a number
of some type.  I'll be quite happy if the error type you associate to your
`TryFrom` implementation is `Box<dyn std::error::Error>`

Then I'd like for you to write a function which takes a filename and returns
a Result<Vec<EnumName>, Box<dyn std::error::Error>> which opens and reads the file
(whichever approach you prefer) and then converts the lines into those EnumName
instances using the implementation above, returning a vector.  It should stop
on the first error, returning that, rather than trying to continue.

At this point, I'd like you to print out the result of parsing your input by
means of `println!("{:?}", thatvector);` to demonstrate that you've parsed
your input usefully.  Do this from your main() function which will therefore
also need to return an error result in order to allow you to use the `?` operator.
Also remember that you'll need to derive debug for your enum to make this possible.

Next I'd like you to define a struct to represent scores.  Each input line is,
in fact, a score from a test.  I'm not telling you if small numbers are good
or bad, they are simply scores.  I want your struct to contain a number of
values: a running total of all the scores you've seen, and a count of those
scores. If a line lacks a score, then it means that person didn't take the test
and you should not count a score for that line.  Instead you should count it
as a missed test, making a third value for your struct to store.

I will want this struct to have an implementation of `std::default::Default`
and for you to use this instead of writing a `ScoreStruct::new` function which
takes no arguments.
>
You'll want some nice methods on that struct such as `add_score` or `missed_test`>
though you likely won't need accessor methods for the scores.>

Finally I'd like you to use `HashMap` to map from a person's name to an
accumulation of their scores.  Ideally you should be using the `Entry` API on
the map to make your code nicer.

Ensure you can print the `HashMap` using `println!("{:?}", fullmap);`

Finally, demonstrate the full scores by iterating through the map, outputting
likes such as:

    Daniel Silverstone took N tests, with a total score of M.  They missed F tests.

Ensure that you use `test` vs `tests` with respect to pluralisation like we did
in the ten green bottles program.  It might be nice if this were done as an
implementation of `std::fmt::Display` on the score structure, allowing a statement
of the form: `println!("{} took {}", hashkey, hashvalue);` to be the core of the
iteration loop over the hash map.

Phew, that was a long one.  Take a moment to go back over your code, add comments
if you feel anything isn't entirely clear, and clean things up before you submit.
*/

use std::collections;
use std::convert;
use std::convert::TryFrom;
use std::default;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
enum TestScore {
   Attended(String, u32),
   Skipped(String),
}

impl TestScore {
    fn name(&self) -> String {
        match self {
            TestScore::Attended(name, _) => name,
            TestScore::Skipped(name) => name,
        }
        .to_string()
    }
}

impl convert::TryFrom<&str> for TestScore {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split(':').collect::<Vec<_>>().as_slice() {
            [name] => Ok(TestScore::Skipped(name.trim().to_string())),
            [name, score] => Ok(TestScore::Attended(name.trim().to_string(), score.parse()?)),
            _ => Err(format!("Couldn't parse {}", value).into()),
        }
    }
}

#[derive(Debug)]
struct CumulativeScore {
    total_attended: u32,
    total_points: u32,
    total_skipped: u32,
}

impl default::Default for CumulativeScore {
    fn default() -> Self {
        Self {
            total_attended: 0,
            total_points: 0,
            total_skipped: 0,
        }
    }
}

impl CumulativeScore {
    fn add_score(&mut self, test: TestScore) -> &Self {
        if let TestScore::Attended(_, score) = test {
            self.total_points += score;
            self.total_attended += 1;
        } else if let TestScore::Skipped(_) = test {
            self.total_skipped += 1;
        };

        self
    }
}

impl fmt::Display for CumulativeScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let tests = |n| if n == 1 { "test" } else { "tests" };
        write!(
            f,
            "Took {} {}, with a total score of {}. Missed {} {}.",
            self.total_attended,
            tests(self.total_attended),
            self.total_points,
            self.total_skipped,
            tests(self.total_skipped)
        )
    }
}

fn parse_scores_from_file(file_name: &str) -> Result<Vec<TestScore>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&file_name)?;
    content.lines().map(TryFrom::try_from).collect() /* very cool FromIter: Vec<Result> -> Result<Vec> !!! */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = env::args().nth(1).ok_or("No file name provided!")?;
    let scores = parse_scores_from_file(&file_name)?;
    println!("scores: {:?}", scores);

    let mut board: collections::HashMap<String, CumulativeScore> = collections::HashMap::new();
    for score in scores {
        board.entry(score.name()).or_default().add_score(score);
    }

    for (name, cumulative_score) in board.iter() {
        println!("{}: {}", name, cumulative_score)
    }

    Ok(())
}
