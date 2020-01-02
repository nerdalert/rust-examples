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
    println!("The add_one one closure results in {}", n);

    // Example 2:
    let example_closure = |mut num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num = &num + 19;
        num
    };
    let before_closure = 10;
    println!("Value before the closure: {}", before_closure);
    let after_closure = example_closure(before_closure);
    println!("Value after the closure: {}", after_closure);

    // Example #3
    let msg: &str = "Hi!";
    let say_hi = || {
        println!("{}", msg);
    };
    say_hi();

    // Example #4 (basically a shortcut function)
    // |x| is the parameter being passed into the closure
    let num = 5;
    let math_op = |x| x * x;
    let sq = math_op(num);
    println!("{}^{} equals -> {}", num, num, sq);

    // Example #5 variables are availble
    // from outside of the closure scope
    let a = "hello".to_string();
    let fs = || {
        println!("we got {}", a);
    };
    fs();
}

/*
Example output:
--------------
The add_one one closure results in 6
Value before the closure: 10
calculating slowly...
Value after the closure: 29
Hi!
5^5 equals -> 25
we got hello
*/
