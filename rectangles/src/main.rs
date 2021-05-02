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

    // :#? in {} denotes pretty printing for objects that don't necessarily implement
    // std::fmt::Display - they will still need to implement Debug or derive from it
    println!("rect3 is {:#?}", rect3);


    println!("The are of rectangle 3 is {} square pixels",
        // '&' because the function expects a reference to an object/struct
        // not an instance of an object/struct
        struct_area(&rect3)
    );
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
