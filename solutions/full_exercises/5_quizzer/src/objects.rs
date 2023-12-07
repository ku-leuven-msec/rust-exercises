//SOLUTION: this file implicitly defines a module `objects` containing these file's contents

use serde::{Serialize, Deserialize};
use crate::config::*;

//TODO you will have to derive the `Serialize` and `Deserialize` traits, see the docs
//SOLUTION: the given JSON file can be deserialized automatically to these structures because the data member names and types match the ones in the JSON file
//You can deviate from this using field attributes, for example:
// - with `#[serde(rename = "name")])` you can use a different data member name
// - with `#[serde(deserialize_with = "path")]` you can provide a custom deserialize function for a data member to convert the string representation to a data structure
// - see https://serde.rs/field-attrs.html for more
//The `serde_with` crate provides more helpers for custom (de)serialization, for example, to use existing `Display` and `FromStr` trait implementations
#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    //SOLUTION: because this struct only has one field, you could as well use `Vec<QuizItem>` as the root structure instead of defining a seperate struct
    //SOLUTION info: Rust has the `type` keyword to create type aliases (like `typedef` in C)
    pub quiz: Vec<QuizItem>
}

//SOLUTION: all containing data structures down the tree also have to derive `Serialize` and `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizItem {
    pub question: String,
    pub answers: [Answer; ANSWERS_SIZE]
}

//SOLUTION
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
    pub is_correct: bool
}


