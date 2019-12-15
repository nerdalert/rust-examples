/// Vectors are re-sizable arrays. Like slices, their size is not
/// known at compile time, but they can grow or shrink at any time.
fn main() {

    let ibm = "ibm".to_string();

    // Example of creating a String Vector
    let providers = vec![
        "google".to_string(),
        "amazon".to_string(),
        "microsoft".to_string(),
        "ibm".to_string(),
    ];

    // check if the vector contains the provider
    if providers.contains(&ibm) {
        println!("Provider -> {} found in the Vector", ibm);
    } else {
        println!("{} not found", ibm);
    }

    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());
}

/*
The output is:
-------------
Provider -> ibm found in the Vector
Collected (0..10) into: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
Initial vector: [1, 2, 3]
Push 4 into the vector
Vector: [1, 2, 3, 4]
Vector size: 4
Second element: 2
Pop last element: Some(4)
*/