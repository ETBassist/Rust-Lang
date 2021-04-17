fn main() {
    let mut counter = 0;
    let result = loop { // can return a result from a loop and assign to variable
        counter += 1;
        println!("Again! {}", counter);
        if counter == 10 {
            break counter * 2; // performs final modification and then breaks & returns
        }
    };

    println!("The result is: {}", result);
    liftoff();

    iterate();
}

fn liftoff() {
    for number in (1..4).rev() { // lower bound inclusive, upper bound exclusive: 1-3
        println!("{}!", number);
    }

    println!("LIFTOFF");
}

fn iterate() {
    let a = [10, 20, 30, 40, 50];
    
    for (index, number) in a.iter().enumerate() {
        println!("The value of a at index {} is {}", index, number);
    }
}
