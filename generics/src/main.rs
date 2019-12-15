/// You can use generics to create definitions for items like
/// functions signatures or structs, which can then be used with
/// different concrete data types.
struct Point<T> {
    x: T,
    y: T,
}

trait GetSound {
    fn get_sound(&self) -> String;
}

struct Cat {
    sound: String,
}
impl GetSound for Cat {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

struct Bell {
    sound: String,
}
impl GetSound for Bell {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

fn make_sound<T: GetSound>(t: &T) {
    println!("{}!", t.get_sound())
}

fn main() {
    // Example 1
    let point_a = Point { x: 0, y: 0 }; // T is a int type
    let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type

    // Example 2, polymorphism example
    let kitty = Cat {
        sound: "Meow".to_string(),
    };
    let the_bell = Bell {
        sound: "Ding Dong".to_string(),
    };

    make_sound(&kitty); // Meow!
    make_sound(&the_bell); // Ding Dong!
}
