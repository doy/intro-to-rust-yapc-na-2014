extern crate debug;

enum List<T> {
    Cons(T, List<T>),
    Nil
}

fn main () {
    let l = Cons(1, Cons(2, Cons(4, Nil)));
    println!("{:?}", l);
}

