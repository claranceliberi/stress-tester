use hyper::{Body, Method, Request, Client};

async fn post_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let req = Request::builder()
    .method(Method::POST)
    .uri("http://173.249.46.156:8080/abunzi_mis_service/api/minCases/addCase")
    .header("content-type", "application/json")
    .header("Authorization", "Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ1bXVtYWoiLCJleHAiOjE2MjU3OTkwNTksImlhdCI6MTYyNTc4MTA1OX0.V_ZGohPVN9cedowOGaoYRwNMIZpJwVZ5tHw6v0QyliD2bfVn_VQSd-2N1EZRshao_q4szo6RUWj1FKy8WzfuXQ")
    .body(Body::from(r#"{"library":"hyper"}"#))?;

    // We'll send it in a second...

    let client = Client::new();

    println!("in method");
    // POST it...
    let resp = client.request(req).await?;

    println!("{:?}", resp);

    Ok(())
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main()  {

    println!("started posting...");
    post_data().await;
    println!("ended posting...");
    // println!("Response: {}", resp.status());

    // Ok(())
}

