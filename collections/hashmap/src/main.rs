// //hashmap: key-value pairs --> insert, get, remove, clear

// use std::collections::HashMap;

// fn main() {
//     let mut users = HashMap::new();
//     users.insert(String::from("Rohith"), 21);
//     users.insert(String::from("Mohith"), 31);

//     println!("{:?}", users);

//     let first_user = users.get("hahaha"); //returns an option.
//     match first_user {
//         Some(val) => println!("{}", val),
//         None => println!("Key doesn't exist.")
//     }
// }

use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();
    vec.push(("Rohith", "lfs"));
    vec.push(("Rohith", "ssps"));
    vec.push(("Abhinav", "Chaitanya"));
    vec.push(("Vaik", "DAV"));
    vec.push(("Vaik", "lfhs"));

    let result = unique_from_vec(vec);
    println!("{:?}", result);
}

fn unique_from_vec(vec: Vec<(&str, &str)>) -> HashMap<String, Vec<&str>> {
    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    for (key, value) in vec {
        let new_vec = vec![value];
        if map.contains_key(key) {
            let existing_vec = map.get_mut(key);
            match existing_vec {
                Some(val) => val.push(value),
                None => match map.insert(key.to_string(), new_vec){
                    Some(val) => println!("Replaced old value"),
                    None => println!("Inserted new val")
                }
            }
        }
    }
    return map;
}
