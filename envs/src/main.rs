use std::env;

fn main() {
    let k = "HOME";
    match env::var(k) {
        // print the contents of 'echo $HOME'
        Ok(v) => println!("Contents of ${} -> {}", k, v),
        Err(e) => println!("ENV variable {} not found: {}", k, e),
    }
}
