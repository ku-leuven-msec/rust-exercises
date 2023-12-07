//SOLUTION: this is the root file for the library part of the crate
//By calling this file `lib.rs`, Cargo will automatically build (without changes to `Cargo.toml`) a Rust-specific static library at target/<debug_or_release>/lib<crate_name>.rlib
//See https://doc.rust-lang.org/reference/linkage.html for different "crate_types", for example, dynamic libraries (either Rust-specific or not)

//SOLUTION: you can only import modules into other crates or targets within a crate (like our binary) if they are declared public (`pub`)
//The `config`  module is only accesses from with this libary itself, so it can remain private
//However, members (data or functions) that are used outside the module still need to be declared public
mod config {
    //SOLUTION: for simplicity, we hardcoded the number of questions per game and the number of possible answers per question
    pub const QUESTIONS_PER_ROUND: usize = 3;
    pub const ANSWERS_SIZE: usize = 4;
    pub const QUIZ_FILENAME: &str = "quiz.json"; //path relative to the current directory
}

//SOLUTION: this defines the `objects` module, and declares it public
//Cargo will by default look for `storage.rs` at certain locations (see the docs) to get the module's contents
pub mod objects;

pub mod storage {
    use std::{fs::File, io::{Read, Write}};
    use crate::{objects::*, config};

    pub fn fetch_quiz() -> anyhow::Result<Quiz> {
        //TODO open the json file quiz.json (see https://doc.rust-lang.org/std/fs/struct.File.html), read its contents, and use the `serde_json` crate to automatically parse it and create the appropriate object structure
        //bonus TODO: validate the deserialized data
        //SOLUTION: we use the `anyhow` crate for easy error propagation
        //Its `anyhow::Error` type behaves like a trait object for the `std::error::Error` trait so you can use the `?` operator for any error type that implements that trait
        let mut file = File::open(config::QUIZ_FILENAME)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let quiz: Quiz = serde_json::from_str(&contents)?;

        //SOLUTION: we did not implement the bonus TODO above, apart from this quick sanity check (it's a fatal error if the quiz is too small)
        if quiz.quiz.len() < config::QUESTIONS_PER_ROUND {
            Err(anyhow::Error::msg(format!("Each round has {} questions but this quiz only has {}", config::QUESTIONS_PER_ROUND, quiz.quiz.len())))?
        }

        Ok(quiz)
    }

    pub fn store_quiz(quiz: Quiz) -> anyhow::Result<()> {
        //TODO convert the quiz back to a string and save it to the quiz.json file
        //SOLUTION
        let mut file = File::create(config::QUIZ_FILENAME)?;
        let s = serde_json::to_string(&quiz)?;
        //SOLUTION: the `write!` macro is the easiest way to write formated data to anything that implements the `std::fmt::Write` or `std::io::Write` trait (like `File` does) (see https://doc.rust-lang.org/std/macro.write.html)
        write!(file, "{}", s)?;
        Ok(())
    }
}

pub mod game {
    use std::str::FromStr;
    use rand::seq::SliceRandom;
    use crate::objects::*;
    use crate::config::*;

    //SOLUTION: only propagate fatal errors
    pub fn play(quiz: &Quiz) -> anyhow::Result<()> {
        let mut score = 0;

        //loop: ask_question - read_answer - check_answer
        //SOLUTION: we use the `rand` crate's `choose_multiple` function in the `rand::seq::SliceRandom` trait (which the crate implements for slices) to get a `amount`-permutation of the length of the slice
        let quiz_items: Vec<&QuizItem> = quiz.quiz.choose_multiple(&mut rand::thread_rng(), QUESTIONS_PER_ROUND).collect();

        for (asked_questions, quiz_item) in quiz_items.iter().enumerate() {
            //SOLUTION: print question and possible answers
            println!("QUESTION {}: {}", asked_questions, quiz_item.question);
            for (j, answer) in quiz_item.answers.iter().enumerate() {
                println!(" > ANSWER {}: {}", j, answer.answer);
            }

            //SOLUTION: read user input
            println!("Which answer is correct?");
            //SOLUTION: loop until the user gives a parcable answer
            let answer = loop {
                let mut input = String::new();
                //SOLUTION: unable to read from stdin is considered fatal
                std::io::stdin().read_line(&mut input)?;
                //SOLUTION: we assume the answer is given in base 10
                match u32::from_str_radix(&input.trim(), 10) {
                    //SOLUTION info: the `if` part below is called a "guard", `break` returns the value the `match` evaluates to
                    Ok(number) if number < 4 => break number,
                    //SOLUTION: parsable but out of bounds
                    Ok(_) => println!("That is not a valid answer index. Try again"),
                    //SOLUTION: not parsable
                    Err(_) => println!("Could not parse your input. Try again")
                }
            };

            //SOLUTION: check answer
            if quiz_item.answers[answer as usize].is_correct {
                score += 1;
                println!("CORRECT!");
            } else {
                println!("INCORRECT");
            }
        }

        println!("Your score: {}/{}", score, QUESTIONS_PER_ROUND);
        Ok(())
    }


    pub fn create() -> anyhow::Result<Quiz> {
        //SOLUTION
        println!("Questions entering mode. Exit by typing `exit` on the question prompt.");
        let mut question_idx = 0;
        let mut quiz = Quiz { quiz: Vec::new() };

        //loop: read_question - read_answers
        loop {
            println!("Question {}:", question_idx);
            question_idx += 1;
            let mut question = String::new();
            std::io::stdin().read_line(&mut question)?;
            if question.trim() == "exit" { break; }

            //SOLUTION: we create a range from 0 to `ANSWERS_SIZE`, map the indices to `Answers`, and collect into a vector
            //The `?` operator inside a closure returns the error from the closure body, and not from the function
            //Therfore, the resulting vector will have the type `Vec<anyhow::Result<Answer>>`
            //However, `collect` can also collect into a `anyhow::Result<Vec<Answer>>` if we specify this with a type annotation (either next to `answer`'s declaration, or with the tubofish notation next to `collect`)
            let answers = (0..ANSWERS_SIZE).map(|answer_idx| {
                println!(" > ANSWER {}:", answer_idx);
                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer)?;

                let is_correct = loop {
                    println!(" > IS CORRECT?");
                    let mut is_correct = String::new();
                    std::io::stdin().read_line(&mut is_correct)?;

                    match bool::from_str(is_correct.trim()) {
                        Ok(b) => break b,
                        Err(_) => println!("Could not parse your input. Try again")
                    }
                };

                //SOLUTION info: in struct instantiation, you can leave out the member names if the assigned variable has the same name (so `answer: answer` can be shortend to just `answer`)
                Ok(Answer { answer, is_correct })
            }).collect::<anyhow::Result<Vec<_>>>()?;
            //SOLUTION info: the `collect` above also requires a type annotation to know the containter type (`Vec`)
            //It can, however, infer the type of the elements on its own, so using the placeholder `_` makes the type annotation shorter

            quiz.quiz.push(QuizItem {
                question,
                //SOLUTION: for demonstrative purposes, we choose to collect into a vector to benefit from the automatic extraction of the `Result`s
                //The disadvantage is that we have to convert the vector to an array
                //try_into` from the `TryInto` trait is like `Into` for conversions that can fail, in our case when the size do not match
                //In this toy example, we can easily verify that this will never fail, and in case it does fail, there is still the run-time check that creates the `Result`
                //However, it is best practise to have the compiler verify this proposition at compile time whenever possible (and it is possible in this case)
                //So alternatively, it may have been better to use `let answers = [(); ANSWER_SIZE].map(...)` to create an array of type `[anyhow::Result<Answer>; ANSWER_SIZE]` and then convert it to an array of type `[Answer; ANSWER_SIZE]` outside the closure
                answers: answers.try_into().unwrap()
            });

        }

        Ok(quiz)

    }
}
