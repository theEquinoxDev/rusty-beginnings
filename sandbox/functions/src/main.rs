fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // ......................................

    let y = {
        // this curly brackets block that evaluate to 4 is an expression
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // ......................................

    let x = five();

    println!("The value of x is: {x}");

    // ......................................
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions in Rust
// . Statements are instructions that perform some action and do not return a value. Statements do not return a value.
// . Expressions evaluate to a resultant value. Expressions do not include ending semicolons
// NOTE: If we add a semicolon to the end of an expression, it becomes a statement and it will then not return a value.

// #Functions with return values
fn five() -> i32 {
    5  // this is an expression and hence didn't add semicolon and means we want to return a value
}

// #Comments
// Starts with double slash only. 
