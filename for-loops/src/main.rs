fn main() {
    let cat_names = ["fluffy", "muffy", "puffy"];

    for i in cat_names.iter() {
        println!("The kittahs name is -> {}", i)
    }

    for (i, name) in cat_names.iter().enumerate() {
        println!("The index is -> {} and the kittahs name is -> {}", i, name)
    }
}

/*
The output is:
--------------
The kittahs name is -> fluffy
The kittahs name is -> muffy
The kittahs name is -> puffy
The index is -> 0 and the kittahs name is -> fluffy
The index is -> 1 and the kittahs name is -> muffy
The index is -> 2 and the kittahs name is -> puffy
*/
