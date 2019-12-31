/// I recommend clap over any other CLI crates. It seems to be the
/// most maintained at this point. https://docs.rs/clap/2.33.0/clap/
extern crate clap;

use clap::{App, Arg};

fn main() {
    let args = App::new("clap cli example")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .hidden_short_help(true)
                .value_name("FILE")
                .help("Path to the config.yml file (defaults to ./config.yml"),
        )
        .get_matches();
    // Print the values from the user args or the default setter
    let config = args.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
}

/*
Example output:
--------------
// Notice when passing args through 'cargo run' the `--` is required
$> cargo run -- -h
clap cli example

USAGE:
    cli-clap [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

// Default options if no args passed
$> cargo run
Value for config: default.conf

// Passing an explicit arg for the '-c' '--conf' flag
$> cargo run -- --config foo.yml
Value for config: foo.yml
*/
