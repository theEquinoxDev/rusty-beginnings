// ## Structs
// Structs are custom data types that let you group related data together.
// You can define your own structs to represent complex data structures in
// your programs. It is similar to objects in JavaScript.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn basic_struct_example() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-in count: {}", user1.sign_in_count);
}

// >> Mutable Structs
// You can make a struct instance mutable by using the `mut` keyword. This
// allows you to change the values of its fields after it has been created.

fn mutable_struct_example() {
    let mut user2 = User {
        active: true,
        username: String::from("anotheruser"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("newemail@example.com"); // Changing the email field
    println!("Updated Email: {}", user2.email);
}

// -- If the entire struct is mutable, all its fields are mutable as well.

// >> Structs with Function parameters
// You can define functions that take structs as parameters to manipulate or
// access their data.

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// In this example, the build_user function creates and returns a User struct instance. Since the parameters and fields have the same names, we can use field init shorthand syntax. Here, we’re creating a new instance of the User struct, which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function. Because the email field and the email parameter have the same name, we only need to write email rather than email: email.

// Creating Instances from Other Instances with Struct Update Syntax
// You can create a new instance of a struct by using the values from another
// instance. This is done using the struct update syntax, which is similar to
// object spread syntax in JavaScript.

fn struct_update_syntax_example() {
    let user1 = User {
        active: true,
        username: String::from("userone"),
        email: String::from("userone@example.com"), 
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("usertwo@example.com"),
        ..user1 // Using struct update syntax to copy remaining fields from user1
    };
    println!("User2 Username: {}", user2.username);
    println!("User2 Email: {}", user2.email);
}
// In this example, user2 is created by specifying a new email and copying the remaining fields from user1 using the ..user1 syntax. It  creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1. The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1. Now, In this example, we can no longer use user1 after creating user2 because the String type does not implement the Copy trait, and thus ownership of the String data has been moved to user2. Also, if we had any fields in user1 that were of types that implement the Copy trait (like integers or booleans), those fields would still be accessible in user1 after creating user2. 
// Note: Struct update syntax only works when the struct being copied from is not moved. If the struct contains fields that do not implement the Copy trait (like String), those fields will be moved, and the original instance will no longer be usable.

// ## Using Tuple Structs Without Named Fields to Create Different Types
// Tuple structs are a special kind of struct in Rust that look like tuples but
// have a struct name. They do not have named fields — only types and index-based access.

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);
fn tuple_structs_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black Color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Origin Point - X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);
}

// ## Unit-Like Structs Without Any Fields
// Unit-like structs are structs without any fields—they have no data inside them.
// They are useful when you need to implement a trait on some type but don’t
// need to store any data.

struct Marker;

fn unit_like_struct_example() {
    let _m = Marker;
}


// Method Syntax
// Methods are similar to functions. We declare them with fn keyword, and they can take parameters and return values. Unlike functions, methods are defined within the context of a struct (or enum or trait object) and have access to the data of the instance they are called on. 

// Defining Methods (Implementing Structs)
// To define methods for a struct, we use the impl keyword followed by the struct name.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height  // we dont' need a return statement here because the last line does not have a semicolon. 
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn rectangle_methods_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} ",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// Here, we defined a method named area for the Rectangle struct. The &self parameter is a reference to the instance of the struct on which the method is called. This allows us to access the fields of that instance within the method. In the main function, we create an instance of Rectangle named rect1 and call the area method on it to calculate and print the area. 

// Note: Methods can also take additional parameters, just like regular functions. The first parameter is always &self (or &mut self for mutable methods), followed by any other parameters you want to include. 

// Where’s the -> Operator?

// In Rust, the -> operator is used in function and method signatures to indicate the return type of the function or method. It separates the parameter list from the return type. For example, in the method area(&self) -> u32, the -> u32 indicates that this method returns a value of type u32.

// ## Methods with More Parameters
// Methods can take additional parameters beyond &self. Here’s an example:

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn rectangle_compare_example() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// In this example, we defined a method named can_hold that takes another Rectangle as a parameter. The method checks if the current rectangle (self) can completely contain the other rectangle (other). In the main function, we create three Rectangle instances and use the can_hold method to check if rect1 can hold rect2 and rect3. The results printed will be true for rect2 and false for rect3.

// ## Associated Functions
// Associated functions are functions that are defined within the context of a struct (or enum or trait object) but do not take &self or &mut self as their first parameter. They are often used as constructors or utility functions related to the struct.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn rectangle_associated_fn_example() {
    let square = Rectangle::square(20);
    println!("The area of the square is {} ", square.area());
}
// In this example, we defined an associated function named square that creates a Rectangle instance where the width and height are equal, effectively creating a square. We call this function using the :: syntax on the Rectangle struct. In the main function, we create a square with a size of 20 and print its area using the area method.

// ## Multiple impl Blocks
// You can define multiple impl blocks for a single struct. This allows you to organize methods and associated functions into separate sections if desired.
// (Removed duplicate impl blocks; see above for the primary impls)

fn main() {
    basic_struct_example();
    mutable_struct_example();
    struct_update_syntax_example();
    tuple_structs_example();
    unit_like_struct_example();
    rectangle_methods_example();
    rectangle_compare_example();
    rectangle_associated_fn_example();
}

