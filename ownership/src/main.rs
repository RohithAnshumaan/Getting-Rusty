fn main() {
    let string_1 = String::from("I am a string in heap"); //string_1 is the first owner for the string in heap memory.
    println!("string_1 = {}", string_1);
    takes_ownership(string_1);
    println!("string_2 died. The heap memory for the string is freed.");
    //println!("{}", string_1); ---> doesn't compile as string_1 died after transferring ownership to string_2
    //one solution (cloning) ---> string_2 = string_1.clone();
    let mut string_3 = String::from("this is string in heap"); //second solution ---> make it mutable so that ownership can be obtained through returns
    println!("string_3 = {}", string_3);
    string_3 = takes_and_returns_ownership(string_3);
    println!("ownership returned to string_3 from string_4 inside function.");
    println!("String_3 = {}", string_3);
}

fn takes_ownership(string_2 : String) {
    println!("string_1 is dead. Ownership goes to string_2 inside function.");
    println!("string_2 = {}", string_2);
    println!("lifetime of string_2 is limited to this function. once this function is popped from stack memory string_2 dies along with the string content in the heap.")
}

fn takes_and_returns_ownership(string_4 : String) -> String {
    println!("string_3 died. ownership transferred to string_4 inside function");
    println!("string_4 = {}", string_4);
    return string_4; //returning the string back to the main function.
}