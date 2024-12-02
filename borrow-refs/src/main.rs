/*  Rules of borrowing
    -> Only one borrower can exist at all times if that borrower is mutable. ===> &mut variable
    -> Multiple borrowers can exist at all times if those borrowers are immutable. ===> &variable
    -> If the mutable borrower doesn't change or update values, the program compiles even if any further borrows are created.
*/

fn main() {
    let mut s1 = String::from("Hello");
    // let s2 = &mut s1; this creates one mutable borrower hence not allowing further borrows.
    // let a2 = &s1 also not allowed is a mutable borrower already exists
    println!("s1 = {}", s1);
    borrow_str(&s1); //pass by reference
    update_str(&mut s1); //pass by mutable reference
    let s5 = &mut s1; // allowed because previous mutable borrower scope ended after function execution.
    println!("s5 = {}", s5);
    println!("updated s1 = {}", s1);
}

fn borrow_str(str: &String) {
    println!("Borrowed the string to str temporarily from original owner");
    println!("str = {}", str);
}

fn update_str(str: &mut String) {     
    str.push_str(" World");
}
