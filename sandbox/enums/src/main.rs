// Enums are a powerful feature in Rust that allow you to define a type by enumerating its possible variants. Simply put, an enum is a type that can be one of several different variants. Each variant can optionally have associated data. Enums are particularly useful when you want to represent a value that can take on a limited set of possibilities. It is the same as enums in TypeScript.

enum Direction {
    North,
    East,
    South,
    West,
}

fn direction_example() {
    let player_direction = Direction::North;

    match player_direction {
        Direction::North => println!("The player is heading North!"),
        Direction::East => println!("The player is heading East!"),
        Direction::South => println!("The player is heading South!"),
        Direction::West => println!("The player is heading West!"),
    }
}

// Enums with Values
// Enums can also have associated values. This allows you to store additional data with each variant

enum Message {
    Quit, // No associated data
    Move {
        x: i32,
        y: i32,
    }, // Named fields
    Write(String), // Single value
    ChangeColor(i32, i32, i32), // Multiple values
}
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received."),
        Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

fn message_example() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello, Rust!"));
    let msg3 = Message::ChangeColor(255, 0, 0);
    let msg4 = Message::Quit;

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}

// The Option Enum and Its Advantages Over Null Values

// In Rust, the Option enum is a powerful way to represent values that may or may not be present. It is defined as follows:
// enum Option<T> {
//     Some(T), // Represents a value of type T
//     None,    // Represents the absence of a value
// }
// Using Option<T> has several advantages over traditional null values found in other programming languages:
// 1. Type Safety: Option<T> is a distinct type, which means that you cannot accidentally use a null value where a value is expected. This helps catch potential errors at compile time.
// 2. Explicitness: The presence of Option<T> in a function signature makes it
//    clear to the caller that a value may be absent, promoting better understanding of the code.
// 3. Pattern Matching: Rust's powerful pattern matching capabilities allow you to handle both Some and None cases explicitly, leading to more robust and maintainable code.
// 4. No Null Pointer Exceptions: Since Rust does not have null pointers, using Option<T> eliminates the risk of null pointer exceptions, which are common in languages that use null values.
// Overall, the Option enum in Rust provides a safer and more expressive way to handle optional values compared to traditional null values.

fn option_demo() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("Option demo: {:?}, {:?}, {:?}", some_number, some_char, absent_number);
}
// here, Some and None are variants of the Option enum. they are used to represent the presence or absence of a value respectively. In this example, some_number and some_char are Option types that contain values, while absent_number is an Option type that does not contain a value.

// ## The match control flow Construct

// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

// Here's a simple example of how match works:
fn match_number_example() {
    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
// In this example, the match statement checks the value of number against several patterns. If it matches one of the patterns, the corresponding code block is executed. The underscore (_) acts as a catch-all pattern that matches any value not explicitly handled by the previous patterns.

// ## Using match with Enums
// Enums and match work particularly well together. When you use match with an enum, you can handle each variant of the enum explicitly. This is especially useful for ensuring that all possible cases are covered, which can help prevent bugs in your code.
// Here's an example using the Message enum defined earlier:
fn match_message_example() {
    let m = Message::Move { x: 1, y: 2 };
    process_message(m);
}
// In this example, the process_message function uses match to handle each variant of the Message enum. This ensures that all possible message types are accounted for, making the code more robust and easier to maintain.

// ## Patterns that Bind Values
// In Rust, patterns can also bind values to variables. This is particularly useful when working with enums that have associated data. When you match against such an enum, you can extract the associated data and bind it to variables for use within the match arm.
// Here's an example using the Message enum:
// fn process_message(msg: Message) {
//     match msg {
//         Message::Quit => println!("Quit message received."),
//         Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
//         Message::Write(text) => println!("Write message: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
//     }
// }
// In this example, when matching the Message::Move variant, the associated x and y values are bound to the variables x and y, which can then be used within that match arm. Similarly, the text in Message::Write and the RGB values in Message::ChangeColor are also bound to variables for use within their respective match arms. This feature of pattern matching makes it easy to work with enums that have associated data.

// ## Matching with Option<T>
// The Option<T> enum is commonly used in Rust to represent values that may or may not be present. When working with Option<T>, you can use match to handle both the Some and None cases explicitly.

fn main() {
    direction_example();
    message_example();
    option_demo();
    match_number_example();
    match_message_example();
}

