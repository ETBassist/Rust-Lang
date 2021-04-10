fn main() {
    let mut x = 5; // must specify that a variable is 'mutable' if its value will change
    println!("The value of x is: {}", x);
    x = 6; // will throw compiler error if x is not mutable
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    // essentially creating a new variable 'y' each time let is called
    println!("The value of y is: {}", y);

    let spaces = "    ";
    println!("There are {} spaces", spaces.len());
    let spaces = spaces.len();
    // spaces cannot be mutable in this instance; values are mutable, types are not
    // ergo calling .len() returns a number type where spaces is of type string
    println!("Or, there are {} spaces", spaces);
}
