use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    
    // ::new on String type; '::' indicates 'new' function is implemented on the *Type* String
    // and not on a particular instance of String, e.g. class methods (in Ruby)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //& indicates that this is a reference
        .expect("Failed to read line"); // .expect is called on an instance of io::Result;
                                        // will return a compiler warning if .expect is not
                                        // included

    println!("You guessed: {}", guess);
}
