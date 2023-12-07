//In this exercise, you will learn how to edit files, parse command line arguments, use modules, use external dependencies, etc.
//You have to write a basic quiz program (quiz creator and quiz player) which interacts with the user over a command line interface
//This Rust project contains some code to get you started

//The program has two modes you choose using the first argument to the program:
// - question-entering mode: allows for entering multiple-choice quiz questions, with 4 possible answers each, exactly 1 of them being correct. The questions are stored on disk as a JSON file
// - quiz mode: loads stored questions from the JSON file, presents some questions (a limited number per round) one-by-one to the player in random order, reads and verifies the player's input, and presents the score at the end of the game
//Handle errors correctly (i.e. your application does not panic if it encounters any unexpected, but non-fatal situation), and use the
//question-mark (?) operator (see https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator)
//info: the `anyhow` crate (https://crates.io/crates/anyhow) can make error propagation cleaner

//For JSON (de)serialization, use the `serde` crate
//for serde_json related info and EXAMPLES: see the documentation at https://docs.rs/serde_json/latest/serde_json/
//(use strongly types data structures)


//TODO 1 first, add the `serde` and `serde_json` dependencies to your Cargo.toml file:
//  serde = {version = "1.0", features = ["derive"]}
//  serde_json = "1.0"
//(look for the latest version on crates.io)
//Cargo will download the dependencies and build them automatically the next time you build your application
//You may add other dependencies as needed


//SOLUTION: use our defined modules
//the root module of the libary (= the module implicitly defined in lib.rs) has the same name as the library (which defaults to the crate name "quizzer")
use quizzer::storage;
use quizzer::game;
use quizzer::objects::Quiz;

//TODO
//SOLUTION: the `main` function can also return a `Result` which will print the error message, set the exit code to 1, and terminate the program
fn main() -> anyhow::Result<()> {

    //read the command line arguments
    //SOLUTION: take the first argument, if not provided: create and return an `Error` with usage description
    let mode = std::env::args().nth(1).ok_or(anyhow::Error::msg("Usage: ./program_name <mode>"))?;

    //match on the mode
    match mode.as_str() {
        "play" => {
            //SOLUTION: call `fetch_quiz` from our `storage` module in the library
            let quiz: Quiz = storage::fetch_quiz()?;
            println!("deserialized = {:?}", quiz);

            game::play(&quiz)?
        },
        "create" => {
            let quiz = game::create()?;
            storage::store_quiz(quiz)?;
        },
        //SOLUTION: matches in Rust are exhaustive, `_` is the default case
        _ => Err(anyhow::Error::msg("Not a valid mode"))?
    }

    Ok(())
}

//Optional TODO: split your code into an application binary and a library
//for example: logic concerning creating, storing, and loading quiz questions is defined in the library part of your crate, and functionality regarding user input (arg parsing, reading from stdin) is defined in the application code
//To do this, you'll have to follow a certain file structure in your project, see https://doc.rust-lang.org/cargo/guide/project-layout.html
//Both library and application can contain multiple modules, see https://doc.rust-lang.org/rust-by-example/mod/split.html
//SOLUTION: see the file and module structure
