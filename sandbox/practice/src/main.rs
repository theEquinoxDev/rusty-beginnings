// #Loop Practice

/*Print all even numbers from 1 to 100.

Print numbers in reverse: 10 → 1.

Use a for loop to compute factorial of a number.

Generate the first 10 Fibonacci numbers. */

// 1. even numbers

fn main() {
    for i in 1..101 {
        println!("{}", i);
    }
}

// ..............................................................

// 2. print numbers in reverse (10 to 1)

fn main() {
   for i in (1..=10).rev() {  // here ..= means it will include 10 also, if we use .. it will exclude 10.
    println!("{}", i); // Here the {} are used to print the value of i, wrt to js, we use ${i} to print the value of i.
   }
}

// ..............................................................

// 3. factorial of a number using for loop

fn main () {
    let mut factorial = 1; 
    let number = 5; 
    for i in 1..=number {
        factorial *=i;
    }
    println!("Factorial of {} is {}", number, factorial);
}

// ..............................................................

// 4. first 10 Fibonacci numbers

fn main() {
    let mut a = 0; 
    let mut b = 1; 

    println!("First 10 Fibonacci numbers: ");

    for _ in 0..10 {  // here _ is used when we don't need the value of the variable. but we can use any varialbe name if we want to use it. eg for i in 0..10
         // but here we don't need the value of i, so we use _.
        print!("{} ", a);
        let next = a + b; 
        a = b; 
        b = next;
    }
}