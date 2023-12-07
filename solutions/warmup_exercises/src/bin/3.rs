//SOLUTION 1: `append_to_string` mutably borrows the string, the borrow ends when `append_to_string` returns
fn main() {
    let mut s0 = String::new();
    
    append_to_string(&mut s0);

    println!("{}",  s0);

    s0.push_str("!");

    println!("{}", s0);
}

//keep this function
fn append_to_string(s: &mut String) {
    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World");
}

//SOLUTION 2: `append_to_string` takes ownership and gives it back by returning the value
fn main() {
    let mut s0 = String::new();
    
    s0 = append_to_string(s0);

    println!("{}",  s0);

    s0.push_str("!");

    println!("{}", s0);
}

//keep this function
fn append_to_string(mut s: String) -> String {
    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World");
    s
}