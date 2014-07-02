fn main () {
    let mut v = vec![6i, 8i, 22i];
    v.push(58);

    println!("{}:", v.len());
    for value in v.iter() {
        println!("  {}", value);
    }
}

