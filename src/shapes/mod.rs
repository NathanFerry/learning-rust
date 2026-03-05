mod shape;
mod circle;
mod rectangle;

pub use shape::Shape;
pub use circle::Circle;
pub use rectangle::Rectangle;

pub fn shape_main() {
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
