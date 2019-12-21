use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // Example #1
    File::open("foo.txt").expect("Could not find file");

    // Example #2
    let f = File::open("foo.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    println!("Example #2: contents --> {:?}", f);

    // Example #3
    let contents = reader("foo.txt".to_string());
    match contents {
        Ok(contents) => print!("Example #3: Everything is ok --> {}", contents),
        Err(e) => println!("Some errors --> {}", e),
    };

    // Example #4
    let contents = read_from_file("foo.txt".to_string());
    match contents {
        Ok(contents) => print!("Example #4: Everything is ok --> {}", contents),
        Err(e) => println!("Some errors --> {}", e),
    };
}

fn reader(filename: String) -> Result<String, io::Error> {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}

fn read_from_file(filename: String) -> Result<String, io::Error> {
    // The ? operator can abbreviate '.expect("some error msg");'
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/*
Example output:
--------------
Example #2: contents --> File { fd: 3, path: "/<Path to the foo.txt file>/foo.txt", read: true, write: false }
Example #3: Everything is ok --> some text
Example #4: Everything is ok --> some text
*/
