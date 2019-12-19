/// An array is a collection of objects of the same type T, stored in
/// contiguous memory. Arrays are created using brackets [], and their
/// size, which is known at compile time, is part of their type signature
/// [T; size].
fn main() {
    let cat_colors: [&str; 3] = ["grey", "black", "orange"];
    for cat in cat_colors.iter() {
        println!("Cat colors --> {}", cat)
    }

    // Same as above with explicit type setting
    let cat_colors: [&str; 3] = ["grey", "black", "orange"];
    for cat in cat_colors.iter() {
        println!("Cat colors --> {}", cat)
    }

    // another option
    let number_of_cats: [i32; 4] = [1, 2, 3, 4];
    for i in 0..number_of_cats.len() {
        println!(
            "Number of index -> {} and number of cats -> {}",
            i, number_of_cats[i]
        )
    }
}

/*
The output is:
-------------
Cat colors --> grey
Cat colors --> black
Cat colors --> orange
Cat colors --> grey
Cat colors --> black
Cat colors --> orange
Number of index -> 0 and number of cats -> 1
Number of index -> 1 and number of cats -> 2
Number of index -> 2 and number of cats -> 3
Number of index -> 3 and number of cats -> 4
*/
