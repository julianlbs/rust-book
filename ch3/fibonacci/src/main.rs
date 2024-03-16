use std::io;

fn main() {
    println!("Enter the position in the Fibonacci series you want the value.");
    let mut position = String::new();
    io::stdin().read_line(&mut position).expect("Failed to read line");

    let position: u32 = position.trim().parse().expect("Please type a number!");

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 1..position {
        c = a + b;
        a = b;
        b = c;
    }

    println!("The value at position {position} is {c}");
}
