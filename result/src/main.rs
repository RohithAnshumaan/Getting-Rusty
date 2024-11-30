use std::fs::read_to_string;
fn main() {
    let result = read_to_string("hello.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(error) => println!("{}", error) //the program will stop(panic) if errors not handled properly
    }
}
