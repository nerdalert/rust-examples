/// An implementation is an item that associates items with an implementing type.
/// Implementations are defined with the keyword impl and contain functions that
/// belong to an instance of the type that is being implemented or to the type
/// statically.
struct Person {
    first_name: String,
    age: u32,
}

impl Person {
    fn new(first_name: &str, age: u32) -> Self {
        Self {
            first_name: first_name.to_string(),
            age: age,
        }
    }
}

fn make_people() {
    let mut bob = Person {
        first_name: "Bob".to_string(),
        age: 25,
    };
    println!(
        "Person ==> Name: {:?} is Age {} yrs old",
        bob.first_name, bob.age
    );

    // use the constructor to instantiate the new instance of Person
    let mut mary = Person::new("Mary", 35);
    println!(
        "Person ==> Name: {:?}  is Age {} yrs old",
        mary.first_name, mary.age
    );
    mary.age = 36;
    println!("Name: {:?} is now {} yrs old", mary.first_name, mary.age);
}

fn main() {
    make_people()
}

/*
The output is:
-------------
Person ==> Name: "Bob" is Age 25 yrs old
Person ==> Name: "Mary"  is Age 35 yrs old
Name: "Mary" is now 36 yrs old
*/
