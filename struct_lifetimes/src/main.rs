
#[derive(Debug)]
struct User <'a>{
    name : &'a str
}

fn main() {
    let name = String::from("Rohith");
    let user = User { //lifetime of user depends on lifetime or the name variable as it is using it's reference.
        name: &name //hence we need to define lifetimes
    };
    println!("{}", user.name);
}
