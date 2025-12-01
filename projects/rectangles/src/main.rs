// 1st way..

// fn main () {
//     let width = 30; 
//     let length = 50; 
//     println!("The area of rectangle is: {} ", area(width, length));
// }

// fn area(width: i32, length: i32) -> i32 {
//     width * length
// }

// 2nd way.. Refactoring with Tuples

// fn main () {
//     let rect1 = (30, 50);
//     println!("The area of rectangle is: {} ", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // 3rd way.. Refactoring with Structs

// struct Rectangle {
//     width: u32, 
//     length: u32, 
// }

// fn main () {
//     let rect1 = Rectangle {
//         width: 30, 
//         length: 50, 
//     };
//     println!("The area of rectangle is: {}", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.length
// }

// Here, we defined a struct named Rectangle with two fields: width and length.
// In the main function, we create an instance of Rectangle named rect1 and initialize its fields.
// The area function now takes a reference to a Rectangle instance and calculates the area using its fields.

// Adding Useful Functionality with Derived Traits

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1}");  // This line will cause a compile-time error because Rectangle does not implement the Display trait.
//     println!("rect1 is {rect1:?}");// This line will also cause a compile-time error because Rectangle does not implement the Debug trait.
// }

// So, to fix this issue, we can derive the Debug trait for the Rectangle struct by adding #[derive(Debug)] above its definition: 

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}"); // Now this will work correctly and print the debug representation of rect1.
//     println!("rect1 is {:#?}", rect1); // Pretty print format
// }

// Using the dbg! Macro for Quick and Easy Debugging

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// The dbg! macro prints the value of the expression along with the file name and line number where it was called.
// In this example, dbg!(30 * scale) will print the result of the multiplication before assigning it to the width field of rect1.
// Similarly, dbg!(&rect1) will print the debug representation of the rect1 instance.
// This is particularly useful for quick debugging without needing to set up more complex logging or debugging tools.


