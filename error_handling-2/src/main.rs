use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

static RANDOM_LYRICS: &str = "
    When there's lightning, 
    you know it always brings me down
    'Cause it's free and I see that it's me
    Who's lost and never found
    I cry out for magic, I feel it dancing in the light
    It was cold, I lost my hold
    To the shadows of the night
    No sign of the morning coming
    You've been left on your own
    Like a rainbow in the dark
    A rainbow in the dark
";

// When you return an error from the main function
// it allows you to return errors and use the ?
fn main() -> Result<(), Box<dyn Error>> {
    // Uncomment the following to see errors returned from main()
    // let _ = File::open("nonexistant-file.txt")?;

    let path_dir = Path::new("./out");
    if !path_dir.exists() {
        println!("Creating directory: {:?}", path_dir);
        fs::create_dir("out")?;
    }

    let path = Path::new("out/lyrics.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `RANDOM_LYRICS` string to `file`, returns `io::Result<()>`
    match file.write_all(RANDOM_LYRICS.as_bytes()) {
        Err(why) => {
            return Err(Box::new(why));
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let nil_file = check_file();
    match nil_file {
        Ok(f) => println!("File: {:?} exists", f),
        Err(e) => println!("Example file not found error: {}", e),
    }

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("out/lyrics.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    Ok(())
}

fn check_file() -> std::io::Result<()> {
    let _ = File::open("nonexistant-file.txt")?;
    Ok(()) // Because of the default return value of Rust functions is an empty tuple/ ()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
