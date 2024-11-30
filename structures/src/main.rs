fn main() {
    let rect = Rectangle {
        length : 10,
        breadth : 20,
    };
    println!("Area of the rectangle is {}", rect.area());
    println!("Perimeter of the rectangle is {}", rect.perimeter());
    println!("{}", Rectangle::static_function());
}

struct Rectangle {
    length : u32,
    breadth : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
    fn perimeter(&self) -> u32{
        2 * (self.length + self.breadth)
    }

    fn static_function() -> String {
        let out = String::from("This is a static function");
        return out;
    }
}
