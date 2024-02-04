// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.


/*
In Rust, there are several built-in types that you can use. Here are some of the commonly used types:

Primitive Types:

bool: Represents a boolean value (true or false).
char: Represents a Unicode scalar value.
Numeric types:
i8, i16, i32, i64, i128: Signed integers with different sizes.
u8, u16, u32, u64, u128: Unsigned integers with different sizes.
f32, f64: Floating-point numbers with single and double precision.
Compound Types:

Arrays: Fixed-size sequences of elements of the same type.
Tuples: Fixed-size sequences of elements of different types.
References:

&T: Immutable reference to a value of type T.
&mut T: Mutable reference to a value of type T.
Pointers:

*const T: Raw pointer to an immutable value of type T.
*mut T: Raw pointer to a mutable value of type T.
Slices:

&[T]: Reference to a contiguous sequence of elements of type T.
Strings:

String: Dynamically allocated, mutable string.
&str: String slice, a reference to a string.
Option and Result:

Option<T>: Represents an optional value that can be either Some(T) or None.
Result<T, E>: Represents the result of an operation that can be either Ok(T) or Err(E).
These are just a few examples of the types available in Rust. Rust also allows you to define your own custom types using structs, enums, and more.
*/

fn main() {
    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
