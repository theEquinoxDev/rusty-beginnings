fn main() {
    // #Number Types

    let x: i32 = 5; // integer variable of unsigned 32 bit type
    let y: u32 = 10; // integer variable of signed 32 bit type
    let z: f32 = 3.14; // floating-point variable of 32 bit type
    println!("x: {}, y: {}, z: {}", x, y, z);

    // #Boolean Type

    let  is_male: bool = true;
    let mut is_above_18: bool = true; // mutable boolean variable means we can change its value later
    is_above_18 = false; // changing the value of mutable variable

    if is_male {
        println!("You are a male.");
    } else {
        println!("You are not a male.");
    }
    if is_male && is_above_18 {
        println!("You are a legal male.");
    }

    // #Strings
    let greeting: String = String::from("Hello, Aditya!");
    println!("{}", greeting);


    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // // println!("The value of y is: {y}");
    // let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
}
