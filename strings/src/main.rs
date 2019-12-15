// Strings are unnecessarily complicated imo in Rust
// Best rule of thumb I'm using is:
// use &str as a function parameter or if you want a read-only view of a string;
// String when you want to own and mutate a string.

fn main() {
    // using String::
    let some_string = String::from("String using String::");
    println!(
        "The String is -> {} with a Length of -> {}",
        some_string,
        some_string.len()
    );

    // random scenario using %str
    let mut s = String::from("Some String");
    appending_string(&mut s);
}

// In this case you need to mix &str and String
fn appending_string(s: &mut String) {
    s.push_str(" adding more to the string");
    println!("{}", s);
}

/*
The output is:
-------------
The String is -> String using String:: with a Length of -> 21
Some String adding more to the string
*/
