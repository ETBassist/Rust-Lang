fn main() {
    // calculating area with a function that takes two parameters
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of rectangle1 is {} square pixels.",
        area(width1, height1)
    );

    // calculating area with a function that take a tuple as an argument
    let rect2 = (20, 20);

    println!("The area of rectangle 2 is {} square pixels",
        tuple_area(rect2)
    );

    // calculating area with a function that takes a reference to a struct as an argument
    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(8);
    // :#? in {} denotes pretty printing for objects that don't necessarily implement
    // std::fmt::Display - they will still need to implement Debug or derive from it
    println!("rect3 is {:#?}", rect3);


    println!("The are of rectangle 3 is {} square pixels",
        // '&' because the function expects a reference to an object/struct
        // not an instance of an object/struct
        struct_area(&rect3)
    );

    println!("Using the area method on the Rectangle struct, the area is {} pixels",
        rect3.area() // parens are mandatory
    );

    println!("Can rect4 hold rect5? {}", rect4.can_hold(&rect5));
    println!("Can rect5 hold rect6? {}", rect5.can_hold(&rect6));

    println!("The area of 'square' is: {}", square.area());
}

// a function that takes two unsigned 32-bit integers and uses them to calculate area
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// a function that take a tuple and calculates the area using the indices of that tuples elements
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// a function that takes a reference to a struct/object and calculates the area using its attrs
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)] // this allows pretty printing of struct instances for debugging
struct Rectangle {
    width: u32,
    height: u32,
}

// area method (as oppposed to function) 
// methods are functions defined within the context of a struct/object
// essentially syntactically identical, but their first param is always 'self'
impl Rectangle { // begin with impl (implementation) block 
    // &self because Rust infers from the impl block that this is a Rectangle
    // we use a reference because we don't want to take ownership of the struct's values
    // including 'mut' would allow us to modify the values
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    // can declare multiple functions within an implementation block
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function: does not requre 'self' as a parameter
    // use Rectangle::square(int) to call
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// it is valid to separate method definitions into separate 'impl' blocks
