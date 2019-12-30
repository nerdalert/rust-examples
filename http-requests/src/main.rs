use reqwest;
use reqwest::Error;

fn main() {
    // Example #1
    let resp = match reqwest::get("https://www.rust-lang.org") {
        Ok(r) => r,
        Err(e) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", e),
    };
    println!("body = {:?}\n", resp);

    // Example #2
    let response = match get_web() {
        Ok(r) => r,
        Err(e) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", e),
    };
    println!("body = {:?}\n", response)
}

fn get_web() -> Result<reqwest::Response, Error> {
    let request_url = "http://github.com";
    let api_resp = reqwest::get(request_url)?;
    Ok(api_resp)
}

/*
Example output:
--------------
body = Response { url: "https://www.rust-lang.org/", status: 200, headers:  ...
body = Response { url: "https://github.com/", status: 200, headers: ...
*/
