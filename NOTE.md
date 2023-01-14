# Scalar types of rust

## integer

if you declare a `integer` `u8` that cand hold values between **0** and **255**.
And if we give a value outside that range 256,  ***interger overflow***  will occurn which can result
in one of two behaviour. When your compiling in debug mode, Rust include checks for integer overflow that cause
your program to panic at runtime if the behaviour ocurrs.  Rust use the term panicking when a program exits with an error

Although, when you compile in release mode `--release` flag, Rust does not include checks for integer overflow
that cause panics. Instead, if overflow ocurrs, Rust performs two's complement wrapping. 
In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. 
In the case of a u8 , the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic,
but the variable will have a value that probably isn’t what you were expecting it to have.
Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods
provided by the standard library for primitive numeric types:

- wrap in all modes with `wrapping_*` method, `such as wrapping_add` 
- Return the `None` value if there is overflow with the `checked_*` methods  
- Return the value  and a Boolean indicating wheter there was overflow with the `overflowing_*` methods
- Saturate at the value's minimum or maximum values with the `saturating_*` methods.  

