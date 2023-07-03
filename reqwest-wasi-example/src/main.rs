use std::collections::HashMap;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:7777/")
        .await
        .expect("Could not get response")
        .text()
        .await?;
    println!("Response body from server: {:#?}", resp);
    Ok(())
}
