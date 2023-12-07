//This is an exercise on using iterators (https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//The file "bee.txt" is read and various operations must be performed on the contents.
//All characters in the provided file are ascii characters. 
//The operations that must be implemented are:
//  - All non-alphabetical characters are discarded.
//  - All words need to be converted to lower case.
//  - The corruptions in the file must be removed. (corruptions are strings: "CORRUPTION")
//  - All Strings that correspond with the keys in `replace_map` must be replaced with their values.
//  - The keys in `words_to_count` must be counted and stored in the data structure.
//All the operations must be done in one pass using a single iterator, for loops are not allowed.
//The result must be a new String of all yielded words with spaces seperating them.
//hint: you'll have to convert the character iterator to a word iterator along the way.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use itertools::Itertools;

fn main() {
    let replace_map = HashMap::from([("barry", "larry"), ("stairs", "ramp"), ("yellow", "purple"), ("breakfast", "elevenses")]);
    let mut words_to_count = HashMap::from([("bee", 0), ("honey", 0), ("adam", 0), ("defenestration", 0)]);

    let mut file  = File::open("bee.txt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let mut outside_acc = String::new();

    //SOLUTION: see the documentation for a description of all iterator functions
    let new_contents = contents.into_iter()
        //SOLUTION: make lowercase
        .map(|b| {
            assert!(b.is_ascii());
            (b as char).to_ascii_lowercase()
        })
        //SOLUTION: filter non-alphabetical characters
        //In this stage, we also accumulate the characters and yield whole words (words are seperated by anything but alphabetic characters)
        //We represent the words as seperate String objects to simplify later stages
        //There are a number of approaches, each with their own drawbacks: 
        //APPROACH 1: the sad thing about this particular implementation is that it requires outside state (the `outside_acc` variable), which is usually not what you want with iterators given its decent from functional programming
        .filter_map(|c| {
            if c.is_alphabetic() {
                outside_acc.push(c); 
                None
            } else if outside_acc.len() != 0 {
                let old_acc = outside_acc.clone(); 
                outside_acc.clear();
                Some(old_acc) 
            } else { None }
        })
       
        //APPROACH 2: `scan` can, like `fold`, accumulate into an internal accumulator, but unlike `fold`, also yields new elements
        //The unfortunate thing about this solution is that `scan` has to yield a new element every iteration, also when the word in the accumulator is not finished yet
        //We use an `Option` for this and yield `None` when the word isn't finished yet, and later filter out the `None`s and unwrap the `Ok`s
        //However, as documented, when `scan`'s closure returns `None`, the iterator stops
        //We, therefore, use an `Option` inside another `Option`
        .scan(/*initial acc:*/ String::new(), |acc, c| {
            if c.is_alphabetic() {
                acc.push(c);
                Some(None)
            } else if acc.len() != 0 {
                let old_acc = acc.clone();
                acc.clear();
                Some(Some(old_acc))
            } else { Some(None) }
        }).filter(|w| w.is_some()).map(|w| w.unwrap())

        //APPROACH 3: both approaches above are not ideal
        //As far as we know, the standard library, at this point in time, does not provide an iterator function that can easily accomplish this task
        //There is, however, a widely used crate, called "itertools" which has a `group_by` function
        .group_by(|c| c.is_alphabetic()).into_iter()
        //the iterator is divided in groups of characters (that form a word), and groups of whitespaces and punctuation 
        //filter out the latter
        .filter_map(|(c, c_iter)| {
            if c { Some(c_iter.collect::<String>()) } else { None }
        })

        //SOLUTION: filter out the corruptions
        .filter(|w| w != "corruption")
        //SOLUTION: apply the map
        .map(|w| String::from(*replace_map.get(w.as_str()).unwrap_or(&w.as_str())))
        //SOLUTION: keep statistics, `inspect` is like a `for_each` that yields the elements
        //Because we asked to put the counts in the `words_to_count` map, you will have to mutate outside state...
        .inspect(|w| {
            if let Some(count) = words_to_count.get_mut(w.as_str()) { *count += 1; }
        })
        //SOLUTION: collect the elements into the final string
        .fold(/*initial acc:*/ String::new(), |acc, s| acc + &s + " ");


    println!("{}", new_contents);

    assert!(!new_contents.chars().any(|c| (!c.is_alphabetic() && c != ' ') || c.is_uppercase()));

    assert!(!new_contents.contains("corruption"));

    for (k, _) in replace_map {
        assert!(!new_contents.contains(&format!(" {} ", k)), "found: {}", k);
    }

    assert_eq!(words_to_count["bee"], 142);
    assert_eq!(words_to_count["honey"], 79);
    assert_eq!(words_to_count["adam"], 158);
    assert_eq!(words_to_count["defenestration"], 0);
    
    println!("Success");

}
