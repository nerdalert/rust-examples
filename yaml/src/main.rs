/// Parse yaml from the included file config.yml
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    tsdb_prefix: String,
    test_length: u32,
    test_interval: u32,
    grafana_address: String,
    grafana_port: String,
    endpoints: Vec<String>,
}

fn main() {
    let filename = "./config.yml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let config: Config = serde_yaml::from_str(&content).unwrap();
            println!("grafana server --> {}", config.grafana_address);
            println!("grafana port --> {}", config.grafana_port);
            println!("test_length --> {}", config.test_length);
            println!("test_interval --> {}", config.test_interval);
            println!("tsdb_prefix --> {}", config.tsdb_prefix);
            for name in config.endpoints.iter() {
                println!("iterating over the endpoint vector {}", name);
            }
        }
        Err(error) => {
            println!("There was an error opening the configuration file: -> {} Error: -> {}", filename, error);
        }
    }
}

/*
Example output:
--------------
grafana server --> "localhost"
grafana port --> "2003"
test_length --> 5
test_interval --> 20
tsdb_prefix --> latency
iterating over the endpoint vector "8.8.8.8"
iterating over the endpoint vector "aws.amazon.com"
iterating over the endpoint vector "cloud.google.com"
*/

