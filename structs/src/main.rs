struct Person {
    name: String,
    age: u16,
}

fn main() {
    // Create struct with field init shorthand
    let Name = "Julian".to_string();
    let Age = 35;
    let julian = Person {
        name: Name,
        age: Age,
    };
    println!("Name -> {} Age -> {}", julian.name, julian.age);

    // Can also instantiate like so
    let randy = Person {
        name: "Randy".to_string(),
        age: 33,
    };
    println!("Name -> {} Age -> {}", randy.name, randy.age);

    // or even....
    let ricky: Person = Person {
        name: "Ricky".to_string(),
        age: 36,
    };
    println!("Name -> {} Age -> {}", ricky.name, ricky.age);
}

/*
The output is:
-------------
Name -> Julian Age -> 35
Name -> Randy Age -> 33
Name -> Ricky Age -> 36
*/
