// Simple if/else and basic loops
fn if_and_basic_loops_example() {
    let is_even: bool = true;

    if is_even {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }

    // #Loops
    for i in 0..10 {
        print!(" {}", i); // prints numbers from 0 to 9
    }
    println!("");
}

// Iterate over collections and extract first word
fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");

    for ch in sentence.chars() {
        ans.push(ch);
        if ch == ' ' {
            break;
        }
    }
    ans
}

fn first_word_example() {
    let sentence = String::from("my name is aditya");
    let first_word = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}

// Using if in a let statement
fn if_in_let_example() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

// ................................................

// rust has three kinds of loops: loop, while, and for.

// 1. Loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
// Basic infinite loop (commented to avoid running forever)
// fn infinite_loop_example() {
//     loop {
//         println!("again!");
//     }
// }

// # Loop Labels to Disambiguate Between Multiple Loops
// This is useful when you have nested loops and want to specify which loop you want to break out of.



fn loop_labels_example() { 
    let mut count = 0;
    'counting_up: loop { // label for the outer loop
        println!("count = {count}");
        let mut remaining = 10;

        loop { // inner loop
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // breaks out of the inner loop
            }
            if count == 2 {
                break 'counting_up; // breaks out of the outer loop
            }
            remaining -= 1; // decrement remaining by 1
        }

        count += 1; // increment count by 1
    }
    println!("End count = {count}");
}

// This is how the whole function works:- At first, count is initialized to 0. The outer loop starts and prints the current count. Then, the inner loop starts and initializes remaining to 10. It prints the current remaining value. If remaining equals 9, it breaks out of the inner loop. If count equals 2, it breaks out of the outer loop using the label 'counting_up'. Otherwise, it decrements remaining by 1. After the inner loop ends, count is incremented by 1, and the process repeats until count reaches 2. Finally, it prints the end count.

// Here is the dry run explanation of the code: 

// At first, count is 0. The outer loop starts, and remaining is set to 10. The inner loop prints remaining (10), then decrements it to 9. The inner loop prints remaining (9) and breaks out of the inner loop. Count is incremented to 1. The outer loop starts again, and remaining is set to 10. The inner loop prints remaining (10), then decrements it to 9. The inner loop prints remaining (9) and breaks out of the inner loop. Count is incremented to 2. The outer loop starts again, and remaining is set to 10. The inner loop prints remaining (10), then decrements it to 9. The inner loop prints remaining (9) and breaks out of the inner loop. Since count is now 2, the outer loop breaks out using the label 'counting_up'. Finally, it prints "End count = 2".

// 2. While Loop

fn while_loop_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// 3. For Loop
fn for_array_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown_for_rev_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    if_and_basic_loops_example();
    first_word_example();
    if_in_let_example();
    loop_labels_example();
    while_loop_example();
    for_array_example();
    countdown_for_rev_example();
}