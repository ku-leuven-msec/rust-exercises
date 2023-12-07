
struct Student { /*some defining data elements of a student*/ }
struct Teacher { /*some defining data elements of a teacher*/ }

//TODO complete the trait definition and create implementations for the `Student` and `Teacher` type to make the code work
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    //SOLUTION: no default implementation needed
    fn say_something(&self) -> String;

}

//SOLUTION: only define `say_something` and use the default implentation for `say_hi`
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

//SOLUTION: (re)define both functions
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

//don't modify this function
fn main() {
    let s = Student{};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher{};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}
