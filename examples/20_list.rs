extern crate debug;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

fn main () {
    let l = box Cons(1i, box Cons(2i, box Cons(4i, box Nil)));
    println!("{:?}", l);
}

