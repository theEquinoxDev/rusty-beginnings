// ## Structs
//    Structs are custom data types that let you group related data together. You can define your own structs to represent complex data structures in your programs. It is similar to objects in JavaScript.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
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

>> Mutable Structs
// You can make a struct instance mutable by using the mut keyword. This allows you to change the values of its fields after it has been created.

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

-- If the entire struct is mutable, all its fields are mutable as well.

>> Structs with Functions parameters
// You can define functions that take structs as parameters to manipulate or access their data.

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
// You can create a new instance of a struct by using the values from another instance. This is done using the struct update syntax, which is similar to object spread syntax in JavaScript.

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

