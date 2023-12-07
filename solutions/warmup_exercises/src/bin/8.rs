use std::fmt::Debug;

//TODO change this struct so its fields have a generic type T
//SOLUTION
struct Pair<T> {
    x: T,
    y: T,
}

//SOLUTION: `T` is not bound, you can call `new` with any type
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

//SOLUTION: `T` is bound to the required traits, you can only call `cmp_display` and those `Pair`s that comply with the bounds
impl<T: PartialOrd + Debug> Pair<T> {
    //Hint: when you make `Pair` a generic type, the compiler will tell you that the `>=` operator and the formatting string `{:?}` are not implemented for all possible concrete types 
    //You have to restrict all possible concrete types to those that do have implementations for them
    //For the `>=` operator, look at table B-1 at https://doc.rust-lang.org/book/appendix-02-operators.html to see which trait declares the `>=` operator and restrict this function to the generic type T that does implement the required traits (you should not restrict the whole definition of `Pair` but only the `cmp_display` function (using the `impl`s type parameter)) (you will have to split this `impl` block in two to only apply the restriction on the `cmp_display` function and not on the `new` function)
    //Same for the `{:?}` formatting string, see https://doc.rust-lang.org/std/fmt/#formatting-traits
    //These restrictions of generic types are called trait bounds
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}


//info: this is a tuple struct
//TODO implement the required traits (see `cmp_display` function) on this type by using default implementations using the `derive` attribute
//SOLUTION
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

fn main() {
    let pair = Pair::new(Unit(1), Unit(3));

    pair.cmp_display();
}
