fn main() {
    let string = String::from("Let's Get Rusty!");
    println!("{}", get_string_length(string));
}

fn get_string_length(str: String) -> usize {
    str.chars().count()
}
