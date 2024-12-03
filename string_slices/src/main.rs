
/*  Types of Strings
    1. String type ---> String::from("");
    2. String slice ---> $string; (has a view to the original object)
    3. String literal ---> = "hello" (type is still &str, points to a address in the binary file after build. Hardcoded)
 */


fn main() {
    let word = String::from("Hello World");
    let first_word = first_word_from_string(word); //word is consumed inside the function
    println!("{}", first_word);
    let new_word = String::from("Hello World");
    let slice = first_word_slice(&new_word); //use pass a reference into the function so the original word is still valid.
    println!("{}", slice); 
    //slice is an immutable reference of the original word so there cannot be any mutable reference as long as this slice is in scope.
}

fn first_word_from_string(str: String) -> String {  //this is one way to obtain a part of a string.
    let mut ans = String::from("");                 // but it creates a new space in heap to store the part.
    for i in str.chars() {                          // not memory efficient. can lead to inconsistencies if original word is cleared
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}

fn first_word_slice(str: &String) -> &str { // You need to pass reference as we are returning a reference to the original word.   
    let mut index = 0; // If you dont pass reference, the string gets consumed inside the function hence unable to return the    
    for i in str.chars() {  //reference as the string is out of scope after this function.
        if i == ' '{
            break;
        }
        index = index + 1;
    }
    return &str[0..index];
}
