fn main() {
    let mut s = String::from("hello");

    println!("Before passing the reference to the function -> {}", s);
    change(&mut s);
    println!("After passing the reference to the function -> {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
Example output:
--------------
Before passing the reference to the function -> hello
After passing the reference to the function -> hello, world
*/