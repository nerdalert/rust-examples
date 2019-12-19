fn main() {
    // call the hello func
    hello();
}

// print hello
fn hello() {
    println!("hello from the function hello() ");
    hello2()
}

// called by the previous function
fn hello2() {
    println!("hello from the function hello2() ");
}

/*
The output is:
-------------
hello from the function hello()
hello from the function hello2()
*/
