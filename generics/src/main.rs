
//Generics are used to avoid code repetitions. Declaring a generic type allows u to process different types in the same function 
// as long as the operators are valid and the code is valid.

fn main() {
    let largest_num = largest_i32(2, 3); // both these functions are similar except the argument types. So generics can be helpful.
    let largest_char = largest_char('a', 'y');
    println!("{} {}", largest_num, largest_char);
    let largest1 = largest(2, 3);
    let largest2 = largest('w', 'f');
    println!("{} {}", largest1, largest2); 
}

fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else {
        b
    }
}

fn largest_char(a: char, b: char) -> char {
    if a > b {
        a
    }
    else {
        b
    }
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T { //std::cmp::PartialOrd ensures that the types are comparable.
    if a > b {
        a
    }
    else {
        b
    }
}