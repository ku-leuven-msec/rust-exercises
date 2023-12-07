//SOLUTION 1: clone the value, the cloned value has its own ownership independent of the original value
fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

//keep this function
fn print_str(arg: String) {
    println!("{}", arg)
}

//SOLUTION 2: `print_str` only temporarily needs the string, so an (immutable) borrow is sufficient
fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

//keep this function
fn print_str(arg: &String) {
    println!("{}", arg)
}

