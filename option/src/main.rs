fn main() {
    let option_return = find_first_a(String::from("Abhinav"));
    match option_return {
        Some(val) => println!("'a' found at index {}", val),
        None => println!("'a' not found in the string.")
    }
}

fn find_first_a(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
    }

    return None;
}