/// Rust does not have a switch  keyword but has match
/// construct that works like a switch statement.
/// Generally, match can run codes or return a single
/// value. It cannot do both at the same time.
fn main() {
    // Example #1 - Match a number
    let num = 5;
    match num {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        5 => println!("The number is 5"),
        _ => println!("Number not matched!"), // This is a required catch all
    }

    // Example #2 - Now let's match a String
    let name = "Randy";
    match name {
        "Julian" => println!("The name is Julian"),
        "Ricky" => println!("The name is Ricky"),
        "Randy" => println!("The name is Randy"),
        "Bubbles" => println!("The name is Bubbles"),
        _ => println!("Name not matched!"), // This is a required catch all
    }

    // Example #3
    let x: i32 = 3;
    println!(
        "{}",
        match x {
            2 => "two",
            3 => "three",
            _ => "not 2 or 3",
        }
    );

    // Example #4
    match_function(Some(0));
    match_function(Some(1));
    match_function(Some(9));
}

fn match_function(x: Option<i32>) {
    match x {
        Some(0) => println!("We got a 0 "),
        Some(1) => println!("We got a 1 => {:?}", x),
        _none => println!("We got an unknown => {:?}", x),
    }
}

/*
The output is:
-------------
The number is 5
The name is Randy
three
*/
