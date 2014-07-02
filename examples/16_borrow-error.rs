fn foo () -> &int {
    let x = 2;
    return &x;
}

fn main () {
    let x = foo();
    println!("{}", *x);
}

