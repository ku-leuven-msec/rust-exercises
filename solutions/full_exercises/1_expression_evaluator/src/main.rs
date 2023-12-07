//TODO define an enum `Operation` with four possible operations: `Add`, `Sub`, `Mul`, `Div` on two subexpressions

//SOLUTION
enum Operation {
    Add, Sub, Mul, Div
}

//an `Expression` is either an operation on two subexpressions or a literal value
//info: remember that enum variants can contain data: `Op` has named fields (like a struct) and `Value` includes an unnamed signed integer
enum Expression {
    Op {
        op: Operation,
        //info: the size of stack allocatable data structures needs to be known and constant at compile time
        //However, these `left` and `right` members make `Expression` a recursive type which could have an infinite size 
        //TODO fix this issue
        //SOLUTION: you can break this recursion using `Box`, these fields now represent a redirection (pointer) to the heap, and the size of the pointer value is know at compile time
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64)
}


//TODO implement the evaluation of an expression (use pattern matching with the `match` keyword)
//Use integer divition for the `Div` operation
//The return type is a `Result`, which, in this case, is either an i64 on success, or a String with the error message on failure
//Look for techniques to make this function as short as possible, for example: use only a single `match` keyword (look at the pattern and destructuring syntax), and propagate errors to the caller
//This function takes ownership of the given `Expression`, DO NOT change this
//Errors can occur, for example, when dividing by 0
//Bonus TODO: return an appropriate error message on integer over/underflow
//SOLUTION, we did not include the bonus TODO, you can accomplish it using the `checked_add`, `checked_sub`, etc. functions that are defined in the interger types (for example: https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op{op: Operation::Add, left, right} => Ok(eval(*left)? + eval(*right)?),
        Expression::Op{op: Operation::Sub, left, right} => Ok(eval(*left)? - eval(*right)?),
        Expression::Op{op: Operation::Mul, left, right} => Ok(eval(*left)? * eval(*right)?),
        Expression::Op{op: Operation::Div, left, right} => {
            let r = eval(*right)?;
            if r == 0 { Err("ERROR: divide by 0".to_owned()) } else { Ok(eval(*left)? / r) }
        },
        Expression::Value(v) => Ok(v)
    }
}

//SOLUTION
pub fn main() {
    //TODO create an expression with literal value 19
    let expr1 = Expression::Value(19);
    assert_eq!(eval(expr1), Ok(19));
    
    let expr2 = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    };
    //TODO print the result of expr2 only if there were no errors
    //hint: have a look at the `if-let` syntax
    if let Ok(v) = eval(expr2) {
        println!("{}", v)
    }

    let expr3 = Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(99)),
        right: Box::new(Expression::Value(0)),
    };
    //TODO evaluate expr3 and print the result or the error message
    match eval(expr3) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e)
    }

    //TODO create, evaluate, and print the expression `(10 * 9) + (5 * (3 - 4))`
    let expr4 = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9))
        }),
        right: Box::new(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(5)),
            right: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4))
            })
        })
    };

    match eval(expr4) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e)
    }

    //SOLUTION NOTE: when you're sure the `eval` function does not fail, you can use the quick and dirty `unwrap` member function of `Result`:
    //println!("{}", eval(expr4).unwrap());

}
