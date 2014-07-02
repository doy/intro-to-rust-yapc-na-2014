enum Color {
    Red,
    Green,
    Blue,
    Custom(int, int, int),
}

fn print_color (c: Color) {
    match c {
        Red             => println!("#ff0000"),
        Green           => println!("#00ff00"),
        Blue            => println!("#0000ff"),
        Custom(r, g, b) => println!("#{:02x}{:02x}{:02x}", r, g, b),
    }
}

fn main () {
    print_color(Red);
    print_color(Custom(0x12, 0x45, 0xba));
}

