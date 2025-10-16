use reqwest::Client;
use serde_json::json;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to TxnOS Terminal!");

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Input error");

        println!("This is your command: {command}");

        let client = Client::new();

        let response = client.post("http://127.0.0.1:8080")
            .json(&json!({
                "jawn": "that one"
            }))
            .send()
            .await?;

        let response_text = response.text().await?;

        println!("Response: {}", response_text);

    }

    Ok(())
}
