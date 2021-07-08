use hyper::{Body, Method, Request, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    // We'll send it in a second...

    let client = Client::new();

    // POST it...
    let resp = client.request(req).await?;

    println!("Response: {}", resp.status());

    Ok(())
}

