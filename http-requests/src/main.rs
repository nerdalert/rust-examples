use reqwest;

const URL: &str = "http://checkip.amazonaws.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?;
    let body = resp.text().await?;
    println!("Your public address is: {}", body);
    Ok(())
}

/*
Example Output:
--------------
<prints your public ip>
*/