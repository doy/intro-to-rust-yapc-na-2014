struct Point {
    x: int,
    y: int,
}

impl Point {
    fn new (x: int, y: int) -> Point { Point { x: x, y: y } }
    fn x (&self) -> int { self.x }
    fn y (&self) -> int { self.y }
}

fn main () {
    let p1 = Point::new(1, 2);
    println!("({}, {})", p1.x(), p1.y());
}

