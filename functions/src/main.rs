fn main() {
    println!("Hello, world!");
// functions execute in the order they are called
    let x = five(); // statement (no return value)
    // different from Ruby, which returns the value assigned

    // this creates a block, within which x is shadowed and reassigned
    // then used in a calculation 
    let y = {
        let x = 3;
        x + 1 // no semicolon at the end of this line: is an expression, not a statment
              // were there a semicolon it would have no return value
    };

    another_function(x, y);

    let z = plus_one(5);
    println!("The value of z is: {}", z)
}

// functions declarations in Rust: fn function_name_here(args) { block }
// args/parameters are variables that are part of that functions signature
fn another_function(x: i32, y: i32) { // Note: type declaration here is mandatory
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// the '->' denotes that this function will return a value
// as in Ruby, the last line executed is that which is returned
// also requires type declaration of expected return
fn five() -> i32 {
    5 // no semicolon because we want it to return a value
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
