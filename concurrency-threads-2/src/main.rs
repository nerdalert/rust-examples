use std::thread;

fn main() {
    let url_two = "https://golang.org";

    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download_page("https://www.rust-lang.org".to_string()));
    // Example of moving the ownership of this variable into a clusure scope
    let thread_two = thread::spawn(move || download_page(url_two.to_string()));

    // main thread blocked until the two threads join
    println!("\nBlocked until worker threads close..");

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");

    println!("Back to work..");
}

fn download_page(url: String) {
    let sitename = url.clone();
    let response = isahc::get(url);
    match response {
        Err(e) => println!("Get response error: {}", e),
        Ok(o) => println!("Response from: {} was {:?}", sitename, o.status()),
    };
}

/*
Example output:
--------------
Blocked until worker threads close..
Response from: https://golang.org was 200
Response from: https://www.rust-lang.org was 200
Back to work..
*/
