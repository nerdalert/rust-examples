fn main() {
    // 1t example - careful with i being out of bounds
    let array = [10, 20, 30, 40, 50, 60];
    let mut i = 0;

    while i < 6 {
        print!("{}", array[i]);
        print!(" ");
        i = i + 1;
    }

    // 2nd example
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }
}

/*
The output is:
-------------
10 20 30 40 50 60 7
11
19
35
*/
