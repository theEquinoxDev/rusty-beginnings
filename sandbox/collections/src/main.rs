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

 // using the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}"); // tic-tac-toe - the format! macro works similarly to println!, but instead of printing the output to the screen, it returns a String with the contents. the macro! does not take ownership of any of its parameters. 

// Indexing into Strings
// Strings cannot be indexed like arrays or vectors. This is because Rust’s strings are UTF-8 encoded, and a single character can take up more than one byte. Indexing into a string could lead to invalid UTF-8 sequences. Rust doesn't support indexing into strings to prevent this issue. 

// Slicing Strings
// rather than indexing, we can create string slices to reference a portion of a String. A string slice is a reference to a part of a String, and it is represented using the &str type.

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // this slice contains the first two characters of the string
    println!("{s}"); // Зд

// Methods for Iterating Over Strings
    for c in "नमस्ते".chars() {
        println!("{c}"); // prints each character 
    }

    for b in "नमस्ते".bytes() {
        println!("{b}"); // prints the byte values 
    }
    // the chars method returns each Unicode scalar value as a char type, while the bytes method returns the raw bytes that make up the string. 
    // in simple words, chars gives you the characters, and bytes gives you the underlying byte representation of those characters.

// 3. Hash Maps
// A hash map is a collection of key-value pairs. Hash maps are implemented using generics, allowing you to store keys and values of any type that implements the Eq and Hash traits for keys, and any type for values. 

// Creating a New Hash Map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

// Inserting Key-Value Pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

// Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("The score for team {} is {}", team_name, score),
        None => println!("Team {} not found", team_name),
    } 
// here, the get method returns an Option<&V>, which will be Some(&value) if the key exists in the map, or None if it does not.
// Iterating Over Key-Value Pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
// Updating a Value
    // Overwriting a Value
    scores.insert(String::from("Blue"), 25); // this will overwrite the previous value for the key "Blue"
    println!("Updated score for Blue: {}", scores.get("Blue").unwrap()); // using unwrap here means that we either get the desired value or the program will panic if the key does not exist.

    // Adding a Value If the Key Has No Value
    scores.entry(String::from("Green")).or_insert(30);
    println!("Score for Green: {}", scores.get("Green").unwrap());
    // the entry method returns an Entry enum that represents a view into a single entry in the hash map, which may be either occupied or vacant. The or_insert method on the Entry enum inserts the given value if the entry is vacant and returns a mutable reference to the value in the entry.

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // this code counts the occurrences of each word in the given text. The split_whitespace method splits the text into words based on whitespace. For each word, we use entry to get the entry for that word in the hash map, inserting 0 if it does not exist. We then increment the count for that word. 

// Managing Ownership in Hash Maps
// for types that implement the Copy trait, like i32, the values are copied into the hash map. For owned types like String, the ownership of the value is moved into the hash map when inserting a key-value pair. 

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // here, both field_name and field_value are moved into the hash map and are no longer valid after the insert operation.

