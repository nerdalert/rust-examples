/// Traits remind me of interfaces in java for my java fam
///
struct Player {
    first_name: String,
    last_name: String,
}

trait FullName {
    fn full_name(&self) -> String;
}

impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let player_1 = Player {
        first_name: "Roger".to_string(),
        last_name: "Federer".to_string(),
    };

    println!("Player 02: {}", player_1.full_name());
}

//
//trait GetSound {
//    fn get_sound(&self) -> String;
//}
//
//struct Cat {
//    sound: String,
//}
//impl GetSound for Cat {
//    fn get_sound(&self) -> String {
//        self.sound.clone()
//    }
//}
//
//struct Bell {
//    sound: String,
//}
//
//impl GetSound for Bell {
//    fn get_sound(&self) -> String {
//        self.sound.clone()
//    }
//}
//
//
//fn make_sound<T: GetSound>(t: &T) {
//    println!("{}!", t.get_sound())
//}
//
//fn main() {
//    let kitty = Cat { sound: "Meow".to_string() };
//    let the_bell = Bell { sound: "Ding Dong".to_string() };
//
//    make_sound(&kitty); // Meow!
//    make_sound(&the_bell); // Ding Dong!
//}
