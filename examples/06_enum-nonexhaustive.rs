enum Color {
    Red,
    Green,
    Blue,
}

fn main () {
    let c = Red;
    println!("{}", match c { Red => "r", Green => "g" });
}

