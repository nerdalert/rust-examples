/// Rust’s closures are anonymous functions you can save in
/// a variable or pass as arguments to other functions. You
/// can create the closure in one place and then call the
/// closure to evaluate it in a different context. Unlike
/// functions, closures can capture values from the scope
/// in which they’re defined
use std::thread;
use std::time::Duration;

fn main() {
    // Example 1: a one liner
    let add_one = |x| x + 1;
    let n = add_one(5);
    println!(" {}", n);

    // Example 2:
    let example_closure = |mut num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num = num + 19;
        num
    };
    let before_closure = 10;
    println!("Value before the closure: {}", before_closure);
    let after_closure = example_closure(before_closure);
    println!("Value after the closure: {}", after_closure);
}
