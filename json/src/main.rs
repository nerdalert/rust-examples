#[macro_use] // required for the  use of the json! macro on line #26
extern crate serde_json;

use serde_json::Value;

fn extract(json: &Value, path: String, extracted: &mut Vec<(String, String)>) {
    match json {
        | Value::String(s) => extracted.push((path, s.clone())),
        | Value::Null => extracted.push((path, "null".to_string())),
        | Value::Number(n) => extracted.push((path, n.to_string())),
        | Value::Bool(b) => extracted.push((path, b.to_string())),
        | Value::Object(m) => {
            for (k, v) in m {
                extract(v, format!("{}/{}", path, k), extracted);
            }
        }
        | Value::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                extract(v, format!("{}[{}]", path, i), extracted);
            }
        }
    }
}

fn main() {
    let test = json! {
        {
            "things": {
                "pet": {
                    "cat": "muffy",
                    "dog": "fido"
                },
                "servers": {
                    "srv1": "192.168.100.10",
                    "srv2": "10.100.10.50"
                }
            }
        }
    };

    let mut parsed_json = Vec::new();
    let root = "".to_string();
    extract(&test, root, &mut parsed_json);
    println!("{:?}", parsed_json);
}