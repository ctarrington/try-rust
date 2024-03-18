use std::collections::HashMap;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await;

    if let Err(e) = response {
        println!("Error: {}", e);
        return Err(Error::new(std::io::ErrorKind::Other, "Error on request").into());
    }

    let response = response?;
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let json = response.json::<HashMap<String, String>>().await;

    if let Err(e) = json {
        println!("Error: {}", e);
        return Err(Error::new(std::io::ErrorKind::Other, "Error on parse").into());
    }

    let json = json?;
    println!("origin: {}", json["origin"]);

    Ok(())
}
