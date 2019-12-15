fn main() {
    ages(5, 6);

    list_ages(8);

    let cat_names = ["fluffy", "muffy", "puffy"];
    list_cats(&cat_names)
}

fn ages(x: i8, y: i8) {
    println!("The age of {} is: {}", "Fluffy", x);
    println!("The age of {} is: {}", "Muffy", y);
}

// list a range of ages starting at the passed int
fn list_ages(num: i8) {
    for n in num..12 {
        println!("Age of cats -> {}", n);
        if is_odd(n) {
            println!("Age {} is an odd number!", n);
        }
    }
}

// check if the number is odd
fn is_odd(num: i8) -> bool {
    return num % 2 == 1;
}
// str are a little bizarre in Rust............[more dots]
fn list_cats(cats: &[&'static str]) {
    for cat in cats.iter() {
        println!("Cat Names --> {}", cat)
    }
}

/*
The output is:
-------------
The age of Fluffy is: 5
The age of Muffy is: 6
Age of cats -> 8
Age of cats -> 9
Age 9 is an odd number!
Age of cats -> 10
Age of cats -> 11
Age 11 is an odd number!
Cat Names --> fluffy
Cat Names --> muffy
Cat Names --> puffy
*/
