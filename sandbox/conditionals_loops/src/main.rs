fn main() {
    //    let is_even: bool = true;

    //    if is_even {
    //        println!("The number is even.");
    //    } else {
    //        println!("The number is odd.");
    //    }

    // #Loops
    //    for i in 0..10 {
    //      print!(" {}", i); // prints numbers from 0 to 9
    //    }

    // we might want to iterate over some collections like an array, map, strings, etc.
    let sentence = String::from("my name is aditya");
    let first_word = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
