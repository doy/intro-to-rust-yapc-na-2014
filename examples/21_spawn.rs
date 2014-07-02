fn ack (m: uint, n: uint) -> uint {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => ack(m - 1, 1),
        (m, n) => ack(m - 1, ack(m, n - 1)),
    }
}
fn main () {
    let (write, read) = channel();
    let (m, n) = (3, 10);

    spawn(proc () {
        write.send(ack(m, n));
    });

    let mut result = read.try_recv();
    while !result.is_ok() {
        println!(".");
        std::io::timer::sleep(100);
        result = read.try_recv();
    }
    println!("Ack({}, {}) = {}", m, n, result.unwrap_or_else(|e| { fail!(e) }));
}

