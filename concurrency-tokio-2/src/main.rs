// Examples with the latest tokio releases
use std::thread;
use std::time;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("doing some work, asynchronously");
        thread::sleep(time::Duration::from_secs(5));
        // Return a value for the example
        "result of the computation #1"
    });
    println!("doin other stuffs..");
    // Wait for the spawned task to finish
    let res = handle.await;
    println!("got {:?}", res);
}
