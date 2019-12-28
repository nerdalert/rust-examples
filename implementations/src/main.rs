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
            age,
        }
    }

    fn print_first_name(&self) -> String {
        self.first_name.to_string()
    }

    fn add_year(&mut self) {
        self.age += 1
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

fn make_people() {
    let mut bob = Person {
        first_name: "Bob".to_string(),
        age: 25,
    };
    println!(
        "Person => Name: {:?} is Age {} yrs old",
        bob.first_name, bob.age
    );

    // use the constructor to instantiate the new instance of Person
    let mut mary = Person::new("Mary", 35);
    println!(
        "Person => Name: {:?}  is Age {} yrs old",
        mary.first_name, mary.age
    );
    mary.age = 36;
    println!("Name: {:?} is now {} yrs old", mary.first_name, mary.age);

    // Call the method to print a name
    println!("Mary's first name is: {}", mary.print_first_name());
    let old_age = bob.get_age();
    // Permanently add a year with the add_year method
    bob.add_year();
    println!("Bob was {} but is now {} yrs old", old_age, bob.get_age());
}

fn main() {
    make_people()
}

/*
The output is:
-------------
Person => Name: "Bob" is Age 25 yrs old
Person => Name: "Mary"  is Age 35 yrs old
Name: "Mary" is now 36 yrs old
Mary's first name is: Mary
Bob was 25 but is now 26 yrs old
*/
