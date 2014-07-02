fn foo () -> Box<int> {
    let x = box 2;
    return x;
}

fn main () {
    let x = foo();
    println!("{}", *x);
}

