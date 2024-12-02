//vector - dynamic array - push, remove

fn main() {
    let mut vec = Vec::new();
    let vec_macro = vec![1,2,3]; //initializing using macro
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}", vec); // {:?} --> debug crate for printing structs.
    even_filter(&vec)
}

fn even_filter(vec: &Vec<i32>) {
    let mut new_vec = Vec::new();
    for val in vec {
        if val%2==0 {
            new_vec.push(*val);
        }
    }
    println!("{:?}", new_vec);
}
