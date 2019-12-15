/// Run with `cargo run hello world`
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
$> cargo run Hello World
["target/debug/args", "Hello", "World"]
First Arg Passed --> "Hello" Second Arg Passed --> "World"
*/
