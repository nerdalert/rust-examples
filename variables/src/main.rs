/// Variables in Rust are associated with a specific data type.
/// The data type determines the size and layout of the
/// variable's memory, the range of values that can be stored
/// within that memory and the set of operations that can be
/// performed on the variable. By default, variables in Rust
/// are immutable an important component of Rust that encourages
/// you to write your code in a way that takes advantage of
/// the safety and concurrency that Rust offers.
fn main() {
    let a = true;
    println!("a is -> {}", a);
    // same as above, just explicitly declaring the type
    let b: bool = true;
    println!("b is -> {}", b);
    // declare multiple variables in one expression
    let (x, y) = (1, 2);
    println!("x is -> {}", x);
    println!("y is -> {}", y);
    // mut is short for mutable allowing us to change the value of z
    let mut z = 3;
    println!("z is -> {}", z);
    // assign a new value to the variable
    z = 100000;
    println!("z is now -> {}", z);
}

/*
The output is:
-------------
a is -> true
b is -> true
x is -> 1
y is -> 2
z is -> 3
z is now -> 100000
*/
