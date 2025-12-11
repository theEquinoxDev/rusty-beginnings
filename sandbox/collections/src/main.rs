// 1. Vectors (Vec<T>)
// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. Vectors are implemented using generics. 

// Creating a new Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

// Updating a Vector
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

// Reading elements of vectors
// can be done using indexing or the get method

let third: &i32 = &v[2];
println!("The third element is {}", third);

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
// here, at first we use indexing. In indexing, if we try to access an element that is out of bounds, the program will panic and crash. In contrast, using the get method returns an Option type, which will be None if the index is out of bounds, allowing us to handle the situation gracefully.

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}");
    // this program will not compile because we are trying to borrow v as mutable (to push a new element) while we still have an immutable reference to one of its elements (first). Rust's borrowing rules prevent this to ensure memory safety.

// Iterating over the values in a vector
// to access each element in a vector, we would iterate through all the elements rather than use indices to access one at a time. 

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // we can also iterate over mutable references to change the values in the vector
        let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

// Using an Enum to store multiple types
// Vectors only store values that are of same types. This can be inconvenient, so we can use enums. 
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

// Dropping a Vector drops Its Elements
// Like any other struct, a vector is freed when it goes out of scope

{
    let v = vec![1, 2, 3, 4];
} // v goes out of scope and is freed here

// 2. Strings
// Strings are a collection of characters. Rust’s string type is UTF-8 encoded, which means it can represent a wide variety of characters from different languages. 

// Creating a new String
    let mut s = String::new();
// often we will have some initial data we want to put into a String. To do that, we can use the to_string method.
        let data = "initial contents";
        let s = data.to_string();

    // The method also works on a literal directly:
        let s = "initial contents".to_string();

// We can also use the function String::from to create a String from a string literal.
        let s = String::from("initial contents");
// Updating a String

    // Appending to a String with push_str and push
    let mut s = String::from("Aditya");
    s.push_str(", Hello!");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); // here, s2 is still valid because push_str takes a string slice (&str) as an argument, which is a reference to a portion of a String. It does not take ownership of the String itself.

    // the push method appends a single character to a String
    let mut s = String::from("lo");
    s.push('l');

// Concatenation with the + operator or the format! Macro
 let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}"); // Hello, world!