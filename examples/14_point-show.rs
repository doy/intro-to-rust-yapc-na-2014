use std::fmt::{Show,Formatter,FormatError};

struct Point {
    x: int,
    y: int,
}

impl Show for Point {
    fn fmt (&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(write!(f, "({}, {})", self.x, self.y));
        return Ok(());
    }
}

fn main () {
    let p1 = Point { x: 1, y: 2 };
    println!("{}", p1);
}

