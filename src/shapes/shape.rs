pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str {
        "Shape"
    }
}
