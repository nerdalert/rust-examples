extern crate futures;

use futures::Future;
use rand;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    // This is a simple future built into the crate which feel sort of like
    // one-time channels. You get a (sender, receiver) when you invoke them.
    // Sending a value consumes that side of the channel, leaving only the reciever.
    let (tx, rx) = futures::oneshot();

    // We can spawn a thread to simulate an action that takes time, like a web
    // request. In this case it's just sleeping for a random time.
    thread::spawn(move || {
        println!("--> START");

        let waited_for = doing_work();
        println!("+++ WAITED {}", waited_for);
        // This consumes the sender, we can't use it afterwards.
        let result = tx.send(waited_for);
        match result {
            Ok(()) => println!("Sending complete!"),
            _ => println!("We have a problem"),
        };

        println!("<-- END");
    });

    // Now we can wait for it to finish
    let result = rx.wait().unwrap();

    // This value will be the same as the previous "WAITED" output.
    println!("{}", result);
}

// generate a random number to sleep threads
pub fn doing_work() -> u64 {
    // Generate random number in the range [1000, 2000]
    let num = rand::thread_rng().gen_range(1000, 2000);
    let sleepy_time = Duration::from_millis(num);
    thread::sleep(sleepy_time);
    num
}
