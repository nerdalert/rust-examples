/// Usage 'cargo run -- --config=config.yml'
/// Set the path to a config file. If no path
/// is passed then a default will be assigned.
use docopt::Docopt;

const DEFAULT_CONFIG: &'static str = "./config.yml";

const USAGE: &'static str = "
args-2

Usage:

  args-2  [--config=<path_to/config.yml>]

Options:
  -h --help     Show this screen.

";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    let config = args.get_str("--config");
    let config = if config.is_empty() { DEFAULT_CONFIG } else { config };

    println!("{:?}", args);

    // You can conveniently access values with `get_{bool,count,str,vec}`
    // functions. If the key doesn't exist (or if, e.g., you use `get_str` on
    // a switch), then a sensible default value is returned.
    println!("\nPassed Arguments:");
    println!("  config: {}", config);
}

/*
FYI: This library seems a little broken/unmaintained.
It's a popular crate but they even recommend using
something else:
"Consider using clap or possibly structopt instead."

Even the --help is broken here. Welcome to Rust ¯\_(ツ)_/¯

Example Output:
--------------
--config => Plain(Some("config.yml"))
-h, --help => Switch(false)

Passed Arguments:
  config: config.yml
*/