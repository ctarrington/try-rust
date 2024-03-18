use std::collections::HashMap;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Option 1 if let pattern
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

    // Option 2 match pattern
    match reqwest::get("https://httpbin.org/ip").await {
        Ok(response) => match response.json::<HashMap<String, String>>().await {
            Ok(json) => {
                println!("origin: {}", json["origin"]);
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(Error::new(std::io::ErrorKind::Other, "Error on parse").into());
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(Error::new(std::io::ErrorKind::Other, "Error on request").into());
        }
    };

    // Option 3: fafo panic pattern
    let response_map = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("origin: {}", response_map["origin"]);

    Ok(())
}
