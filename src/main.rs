use shapes::{Circle, Rectangle, Shape};

pub mod shapes;

fn main() {
    let rect = Rectangle::new(4.0, 5.0);
    let square = Rectangle::new(4.0, 4.0);
    let circle = Circle::new(3.0);

    println!("Is the rectangle a square? {}", rect.is_square());
    println!("Rectangle area: {}", rect.area());
    println!("Circle area: {}", circle.area());

    let shapes: Vec<&dyn Shape> = vec![&rect, &square, &circle];

    for shape in shapes {
        println!("Shape is {}, area: {}", shape.name(), shape.area());
    }
}
