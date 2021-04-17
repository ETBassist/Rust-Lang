fn main() {
    let number = 6;

    // condition must be/return bool; Rust has no 'falsey' values, e.g. 
    // a string '' will not return true or false
    if number % 4 == 0 { // blocks of code associated with conditionals are called 'arms'
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else { // if no else condition provided will execute nothing and continue on after checking if statement(s)
        println!("number was not divisible by 4, 3, or 2");
    }
    
    println!("The value of if_in_let is: {}", if_in_let(true));
}

fn if_in_let(condition: bool) -> i8 {
    // types here must match: if condition {5} else {"six"} would throw compiler error
    let number = if condition {5} else {6};

    number // no semicolon means this is return of function
}
