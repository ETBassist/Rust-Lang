use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // gen_range is lower bound inclusive, upper bound exclusive;
    // ergo the below is 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        
        // ::new on String type; '::' indicates 'new' function is implemented on the *Type* String
        // and not on a particular instance of String, e.g. class methods (in Ruby)
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // the '&' indicates that this is a reference
            .expect("Failed to read line"); // .expect is called on an instance of io::Result;
                                            // will return a compiler warning if .expect is not
                                            // included
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 'shadow' guess variable and cast it to unsigned 32-bit integer
        // using a 'match' instead of an '.expect' is generally how one handles errors

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // 'match' expression is made up of 'arms' (here being >=, <=, ==)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
