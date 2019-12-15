fn main() {
    let original_number = 30;
    println!("Before the function -> {}", original_number);

    let new_num = add_one(original_number);
    println!("After the function -> {}", new_num);
}

// take a number as a parameter, add 1 and return the new value
fn add_one(x: i32) -> i32 {
    x + 1
}

/*
The output is:
-------------
Before the function -> 30
After the function -> 31
*/