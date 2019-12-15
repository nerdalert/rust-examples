# rust-examples
A collection of Rust examples for learning Rust for beginners to start getting a feel for Rust. I have recently started playing around with Rust and wanted to post code snippets for the basic concepts and mechanics for those who learn by hacking like I do.

# Install Rust

The only prerequisite is to have Rust installed on your machine. Install it with the following commands:

```
# Download and install Rust
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env 
# You can test the installtion with the following
cargo new rust1 --bin
cd rust1/
cargo run ./
```

# Clone the repo and run the example snippets

To get started with the snippets, simply clone the repo:

```
git clone https://github.com/nerdalert/rust-examples.git
cd rust-examples
```

Then cd into any of the directories and run the app with `cargo run`

- Below is an example of how to run the code in this repo:

```
(='o'=) [rust-examples]$ cd hashmap 
(='o'=) [hashmaps]$ cargo run
   Compiling hashmaps v0.1.0 (/Users/brent/rust/tutorials-brent/hashmaps)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/hashmaps`
Server count -> 4
The IP for server1 is -> Some("172.17.0.10")
The new IP for server1 is -> Some("192.168.200.40")
server2 : 192.168.100.10
server1 : 192.168.200.40
server3 : 10.100.10.20
server4 : 10.200.10.30
```

Happy hacking family!

