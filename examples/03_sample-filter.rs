fn main() {
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 1
    // `*` or `/` means multiply or divide by 2

    let program = "+ + * - /";
    let mut accumulator = 0i;

    for token in program.chars().filter(|&x| { x != ' ' }) {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _   => fail!("unknown character"),
        }
    }

    println!("The program \"{}\" calculates the value {}",
              program, accumulator);
}

