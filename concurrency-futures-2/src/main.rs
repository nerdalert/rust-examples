extern crate futures;

use futures::future::join_all;
use futures::sync::oneshot;
use futures::Future;
use rand::Rng;
use std::thread;
use std::time::Duration;

const NUM_OF_TASKS: usize = 10;

fn main() {
    // We'll create a set to add a bunch of recievers to.
    let mut rx_set = Vec::new();

    // Next we'll spawn up a bunch of threads doing 'something' for a bit then sending a value.
    for index in 0..NUM_OF_TASKS {
        // Here we create a future, this is a `oneshot` value which is consumed after use.
        let (tx, rx) = futures::oneshot();
        // Add the reciever to the vector we created earlier so we can collect on it.
        rx_set.push(rx);

        // Spawning up a thread means things won't be executed sequentially, so this will actually
        // behave like an asynchronous value, so we can actually see how they work.
        thread::spawn(move || {
            println!("{} --> START", index);
            let waited_for = doing_work();
            println!("{} +++ WAITED {}", index, waited_for);
            // Here we send back the index (and consume the sender).
            let sender = tx.send(index);
            match sender {
                Ok(()) => println!("Sending complete!"),
                _ => println!("We have a problem"),
            };
            println!("{} <-- END", index);
        });
    }

    // `join_all` lets us join together the set of futures.
    let result = join_all(rx_set)
        // Block until they all are resolved.
        .wait()
        // Check they all came out in the right order.
        .map(|values| {
            values
                .iter()
                .enumerate()
                .all(|(index, &value)| index == value)
        })
        // We'll be lazy and just unwrap the result.
        .unwrap();
    println!("Job is done. Values returned in order: {}", result);
}

// generate a random number to sleep threads
pub fn doing_work() -> u64 {
    // Generate random number in the range [1000, 2000]
    let num = rand::thread_rng().gen_range(1000, 2000);
    let sleepy_time = Duration::from_millis(num);
    thread::sleep(sleepy_time);
    num
}
