fn get_full_name(fname: &str, lname: &str, mname: Option<&str>) -> String {
    match mname {
        Some(n) => format!("{} {} {}", fname, n, lname),
        None => format!("{} {}", fname, lname),
    }
}

fn main() {
    println!("{}", get_full_name("Sgt", "Meowenstein", None));
    println!("{}", get_full_name("Fluffy", "Kitty", Some("Cat")));
}
