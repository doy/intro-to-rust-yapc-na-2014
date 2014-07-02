trait Area {
    fn area (&self) -> f64;
}
struct Rectangle {
    width:  f64,
    height: f64,
}
impl Area for Rectangle {
    fn area (&self) -> f64 { self.width * self.height }
}
struct Circle {
    radius: f64,
}
impl Area for Circle {
    fn area (&self) -> f64 { 3.14 * self.radius * self.radius }
}
fn print_area<T: Area> (shape: T) {
    println!("{}", shape.area());
}
fn main () {
    print_area(Circle { radius: 2.0 });
    print_area(Rectangle { width: 3.2, height: 4.5 });
}

