/// Run with `cargo run first second thrid`
/// "target/debug/args" will always be the first arg[0] in Rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!(
        "First Arg Passed --> {:?} Second Arg Passed --> {:?}",
        args[1], args[2]
    );
}

/*
Example output:
--------------
$> cargo run first second third
["target/debug/args-1", "first", "second", "third"]
First Arg Passed --> "first" Second Arg Passed --> "second"

Note: you will get out of bounds errors from the vector
is you don't pass any arguments.
*/
