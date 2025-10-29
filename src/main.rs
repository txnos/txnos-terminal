use reqwest::Client;
use serde_json::json;
use serde_json;
use std::collections::HashMap;
use repl_rs::{Command, Parameter, Value};
use repl_rs::{Convert, Repl};
use tokio::runtime::Runtime;
use blake2::{ Blake2s256, Digest };


async fn full_send(s: String) -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();

    let mut hasher = Blake2s256::new();

    let response_json = &json!({
            "ver": 1,
            "txn": {
                "from": "abcdef",
                "st": {
                    "obj": "/x/y/z",
                    "chash": "0xabcdef",
                    "nvalue": s
                }
            },
            "sig": "abcdef"
        });

    let response_json_string = serde_json::to_string(response_json)?;
    let response_json_bytes = serde_json::to_vec(response_json)?;

    hasher.update(&response_json_bytes);

    let hasher_result = hasher.finalize();

    let hex_result = hex::encode(hasher_result);

    println!("response_json_string: {}", response_json_string);
    println!("hash: {}", hex_result);

    let response = client.post("http://127.0.0.1:8080")
        .json(&json!({
            "ver": 1,
            "txn": {
                "from": "abcdef",
                "st": {
                    "obj": "/x/y/z",
                    "chash": "0xabcdef",
                    "nvalue": s
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
