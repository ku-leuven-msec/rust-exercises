//This program reads your name through stdin and greets you
//A name can only contain alphabetical characters and cannot be empty

//TODO fix the compiler errors and have a look at the warnings

//This program uses `panic!()` to deal with names that do not comply with the name format
//Using `panic!()` is a quick-and-dirty way to do error handling, however, it has the obvious drawback that it is all-or-nothing: you cannot recover from it (in general)

//TODO change to program so it doesn't panic on malformatted names, and handle the compiler warnings
//We have provided an error type for properly reporting all errors that `get_username` might generate
//Change `get_username` so it returns a `Result<String, MyError>` and handle the errors in `main` (an IOError should quit the program, but after an InvalidName error it should repeat the question to the user)
//hint: have a look at `Result`s `or_else` function, and the `?` operator

//SOLUTION: bring in scope so we can write `io::...` instead of `std::io::...`
use std::io;
//SOLUTION: items from traits (the `flush` and `read_line` functions) can only be used when the trait is in scope
use std::io::Write;
use std::io::BufRead;

enum MyError {
    InvalidName,
    IOError(io::Error),
}

//SOLUTION: we propagate the potential errors to the caller using the `Result` return type
fn get_username() -> Result<String, MyError> {
    print!("Username: ");
    //SOLUTION: `flush` returns a `Result<(), io::Error>`, so we cannot use the `?` operator because the `Err` type (`io::Error`) is different from the functions return `Err` type (`MyError`)
    //We instead want to "replace" the `io::Error` with a `MyError` when it occurs
    //`or_else` can do exactly that: 
    //  - when `flush` succeeds (returns `Ok(())`), the expression evaluaties to the `Ok` value
    //  - when `flush` fails (returns `Err(io::Error)`), the expression evaluates to the result of the closure, in which we create a new `Err` value of the `MyError::IOError` type
    //Now we can use the `?` operator to take out the value of the `Ok` (and discard it since we don't assign it to a variable), or immediatly return the `Err` to the caller
    io::stdout().flush().or_else(|e| Err(MyError::IOError(e)))?;

    let mut input = String::new();
    //SOLUTION: same remarks as above
    io::stdin().lock().read_line(&mut input).or_else(|e| Err(MyError::IOError(e)))?;
    input = input.trim().to_string(); //have a look at the docs to see what `trim` does

    for c in input.chars() {
        if !char::is_alphabetic(c) {
            //SOLUTION: propagate a `MyError` to the caller
            return Err(MyError::InvalidName);
        }
    }

    if input.is_empty() {
        //SOLUTION: same remark as above, but written differently
        Err(MyError::InvalidName)?;
    }

    //SOLUTION: wrap the return value in an `Ok` to comply with the return type of the function
    Ok(input)
}

fn main() {
    //SOLUTION: infinite loop te repeat the question
    loop {
        match get_username() {
            Ok(name) => println!("Hello {name}!"),
            Err(MyError::InvalidName) => {println!("that's not a valid name, try again")},
            //SOLUTION: as specified, on an `IOError` we are allowed to quit the program, we choose to use `panic!`, but you could also, for example, just return from the `main` function 
            Err(MyError::IOError(e)) => panic!("{}", e)
        };  
    }
}
