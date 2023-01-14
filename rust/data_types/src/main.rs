use std::{io, process::exit};
fn main() {
    let mut x = String::new();

    println!("Enter a number");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read the num");
    let num: u8 = match x.trim().parse::<u8>() {
        Ok(number) => number,
        Err(_) => exit(404),
    };

    integer_type(num);
}

fn integer_type(num: u8) -> u8 {
    println!("Your num is {num}");
    return num;
}
