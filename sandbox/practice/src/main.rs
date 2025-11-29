// #Loop Practice

/*Print all even numbers from 1 to 100.

Print numbers in reverse: 10 → 1.

Use a for loop to compute factorial of a number.

Generate the first 10 Fibonacci numbers. */

// 1. even numbers
fn print_evens() {
    for i in 1..=100 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}

// ..............................................................

// 2. print numbers in reverse (10 to 1)

fn reverse_countdown() {
    for i in (1..=10).rev() {  // here ..= means it will include 10 also, if we use .. it will exclude 10.
        println!("{}", i);
    }
}

// ..............................................................

// 3. factorial of a number using for loop

fn factorial_demo() {
    let mut factorial = 1; 
    let number = 5; 
    for i in 1..=number {
        factorial *= i;
    }
    println!("Factorial of {} is {}", number, factorial);
}

// ..............................................................

// 4. first 10 Fibonacci numbers

fn fibonacci_demo() {
    let mut a = 0; 
    let mut b = 1; 

    println!("First 10 Fibonacci numbers: ");

    for _ in 0..10 {
        print!("{} ", a);
        let next = a + b; 
        a = b; 
        b = next;
    }
    println!("");
}

fn main() {
    print_evens();
    reverse_countdown();
    factorial_demo();
    fibonacci_demo();
}