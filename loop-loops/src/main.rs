fn main() {
    let mut count = 0;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        // Without this next block with the `break`,
        // the program will run until a ctrl c is sent
        if count == 5 {
            println!("No more!");
            // Exit this loop
            break;
        }
    }
}

/*
The output is:
-------------
Let's count until infinity!
1
2
three
4
5
No more!
*/
