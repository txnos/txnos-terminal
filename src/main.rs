use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use repl_rs::{Command, Parameter, Value};
use repl_rs::{Convert, Repl};
use tokio::runtime::Runtime;


async fn full_send(s: String) -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();

    let response = client.post("http://127.0.0.1:8080")
        .json(&json!({
            "ver": 1,
            "txn": {
                "from": "abcdef",
                "st": {
                    "obj": "/x/y/z",
                    "chash": "0xabcdef",
                    "nvalue": "fedcba"
                }
            },
            "sig": "abcdef"
        }))
        .send()
        .await?;

    let response_text = response.text().await?;

    println!("Response: {}", response_text);
    
    Ok(())
}

fn tinker_send<T>(args: HashMap<String, Value>, _context: &mut T) -> repl_rs::Result<Option<String>> {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(full_send(args["what"].to_string()));
    println!("Result: {:?}", result);
    Ok(Some(format!("Sent this: {}", args["what"])))
}

fn main() -> repl_rs::Result<()> {
    println!("Welcome to TxnOS Terminal!");

    let mut repl = Repl::new(())
        .with_name("TxnOS Terminal")
        .with_version("0.0.1")
        .with_description("OEM CLI terminal for TxnOS")
        .add_command(
            Command::new("tinker_send", tinker_send)
                .with_parameter(Parameter::new("what").set_required(true)?)?
                .with_help("Send a test transaction to the kernel"),
        );

        repl.run()
}
