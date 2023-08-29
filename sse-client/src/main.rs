use futures_util::stream::StreamExt;
use reqwest_eventsource::RequestBuilderExt;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert("message", "hello");

    // curl -H "Content-Type: application/json" -X POST "http://localhost:5000/sse" -d '{"message":"hello"}'
    let client = reqwest::Client::new();
    let mut stream = client
        .post("http://localhost:5000/sse")
        .json(&map)
        .eventsource()
        .expect("Failed");

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                dbg!(event);
            }
            Err(err) => {
                dbg!(err);
                break;
            }
        }
    }
}
