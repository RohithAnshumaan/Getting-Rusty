enum Shape {
    Square(f32),
    Circle(f32)
}

fn find_area_of_shape(shape: Shape) -> f32{
    match shape {
        Shape::Square(val) =>  val * val,
        Shape::Circle(val) =>  3.14 * val * val
    }
}

fn main() {
    let square = Shape::Square(4.0);
    let circle = Shape::Circle(2.0);
    println!("{}", find_area_of_shape(square));
    println!("{}", find_area_of_shape(circle));
}