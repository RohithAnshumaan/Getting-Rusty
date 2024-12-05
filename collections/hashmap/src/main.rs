use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();
    vec.push(("Rohith", "lfs"));
    vec.push(("Rohith", "ssps"));
    vec.push(("Abhinav", "Chaitanya"));
    vec.push(("Vaik", "DAV"));
    vec.push(("Vaik", "lfhs"));

    //let result = unique_from_vec(vec);
    let result = unique(vec);
    println!("{:?}", result);
}

fn unique_from_vec<'a>(vec: Vec<(&'a str, &'a str)>) -> HashMap<String, Vec<&'a str>> {
    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    for (key, value) in vec {
        let new_vec = vec![value];
        match map.get_mut(key) {
            Some(val) => val.push(value),
            None => match map.insert(key.to_string(), new_vec) {
                Some(val) => println!("Replaced old value - {:?}", val),
                None => println!("Added new entry")
            }
        }
    }
    return map;
}

fn unique<'a>(vec: Vec<(&'a str, &'a str)>) -> HashMap<String, Vec<&'a str>> { //simple way using entry API
    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    for (key, value) in vec {
        map.entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }
    return map;
}