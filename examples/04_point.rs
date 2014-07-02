struct Point {
    x: int,
    y: int,
}

fn main () {
    let p1 = Point { x: 1, y: 2 };
    println!("({}, {})", p1.x, p1.y);
}

