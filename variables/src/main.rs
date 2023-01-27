#![allow(unused)]
fn main() {
    // mut variables declare with keyword mut
    let mut x = 30;

    println!("Variable x = {x}");

    x = x - 5;

    println!("Now variable x = {x}")
}

fn shadowing() {
    let number = 5;
    let number = number + 2;

    {
        let number = number * 2;
        println!("Value of number is: {number}")
    }
    println!("value of the number outside if {number}")
}

fn constant_variable() {
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
}
