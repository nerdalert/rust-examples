/// Rust does not have a switch  keyword but has match
/// construct that works like a switch statement.
/// Generally, match can run codes or return a single
/// value. It cannot do both at the same time.
fn main() {
    // Match a number
    let num = 5;
    match num {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        5 => println!("The number is 5"),
        _ => println!("Number not matched!"), // This is a required catch all
    }

    // Now let's match a String
    let name = "Randy";
    match name {
        "Julian" => println!("The name is Julian"),
        "Ricky" => println!("The name is Ricky"),
        "Randy" => println!("The name is Randy"),
        "Bubbles" => println!("The name is Bubbles"),
        _ => println!("Name not matched!"), // This is a required catch all
    }
}

/*
The output is:
-------------
The number is 5
The name is Randy
*/
