use std::io;

fn main() {
    println!("Welcome to the Guessing Game!"); // println! is a macro that prints text to the console
    println!("Please enter your guess:");

    let mut guess = String::new(); // in rust, variables are immutable by default, so we use "mut" to make it mutable..
    //    The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string

    io::stdin() // io is a module and stdin is a function
        .read_line(&mut guess) // &mut indicates that we are passing a mutable reference to the guess variable. read_line is a method that reads a line from standard input and appends it to the provided string. & indicates that we are passing a reference to the variable rather than the variable itself. references are immutable by default as well.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
