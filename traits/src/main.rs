
//Traits help in defining shared behaviour in an abstract manner. It can be shared by different types.

trait Summary {     // trait definition
    fn summarize(&self) -> String; // abstract function, must be defined if implementation needed.
    fn get_age(&self) -> String { // default implementation
        return String::from("Age not disclosed.");
    }
}

trait Fix {
    fn fixing(&self) -> String {
        return String::from("This is a second trait.");
    }
}

struct User {
    name : String,
    age : u32
}

impl Summary for User { //implementation for User struct
    fn summarize(&self) -> String {
        return format!("I am {}. My age is {}.", self.name, self.age);
    }
    //get_Age function not defined so default implementation.
}
impl Fix for User {}

fn notify(u: &impl Summary) { //Function accepts structs or types with only specific traits as parameters.
    println!("This function is defined for structs or types the have certain traits or implement a certain trait.");
    println!("{}", u.summarize());
}

//TRAIT-BOUNDS --> Under the hood functioning. 
fn notify_1<T: Summary + Fix>(item: &T) { //This generic input needs to implement both summary and fix traits.
    println!("Breaking news! {}", item.summarize());
    println!("{}", item.fixing());
}

fn main() {
    let user = User {
        name : String::from("Rohith"),
        age : 21
    };
    println!("{}", user.summarize());
    println!("{}", user.get_age());
    notify(&user); //valid as user implements the Summary trait.
    notify_1(&user);
}


