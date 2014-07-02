fn main () {
    let mut s = String::from_str("Hello");
    s.push_str(" world");

    println!("{}", s.len());
    println!("{}", s);
}

