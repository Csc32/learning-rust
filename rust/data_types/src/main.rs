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
    floating_points();
}

fn integer_type(num: u8) -> String {
    let num: String = num.to_string();
    println!("The number is {num}");
    return num;
}

fn floating_points() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("The value of x = {}", x.to_string());
    println!("The value of y = {}", y.to_string())
}
