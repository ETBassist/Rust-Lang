fn main() {
    // no 'new' keyword required (unlike Ruby)
    let mut user1 = build_user(String::from("example@example.com"), String::from("a user"));

    let black = Color(0, 0, 0);

    // access attributes of struct via dot notation
    if user1.active {
        println!("User name is: {} and user has signed in {} times.", user1.username, user1.sign_in_count); 
        } else {
        println!("User not active");
    };

    println!("Value of color struct instance 'black' at index 0 is: {}", black.0);
    // reassign attributes with dot notation also
    // instance must be mutable to reassign attributes

    user1.email = String::from("new_email@example.com");

    let _user2 = User { // prefix variable with underscore to mute warnings for unused code
        email: String::from("another@example.com"),
        username: String::from("another user"),
        ..user1 // this shorthand means it will get any values not explicitly supplied from user1
    };
}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct: attrs need not be named, and instead are accessed via '.' + index
struct Color(i8, i8, i8); 

// declare a function that will return a new instance of User struct with email and username args
fn build_user(email: String, username: String) -> User {
    User { // if args and attrs are identically named, shorthand allows plugging arg straight in without ':'
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
