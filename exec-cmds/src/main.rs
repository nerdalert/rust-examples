use std::process::Command;

fn main() {
    // Spawn a process, wait for it to finish, and collect it's output
    let the_output = Command::new("ps")
        .arg("aux")
        .output()
        .ok()
        .expect("Failed to execute.");
    // Encode the resulting data.
    let encoded = String::from_utf8_lossy(the_output.stdout.as_slice());
    print!("{}", encoded);

    // list cwd
    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");
    // Change `ls` to execute in the root directory.
    list_dir.current_dir("/");
    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}

/*
The output is:
-------------
ps -aux
ls
*/
