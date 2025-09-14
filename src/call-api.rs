use serde::Deserialize; //to turn JSON into Rust structs.
use reqwest::Error;
use reqwest::header::{ACCEPT, USER_AGENT};

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main] //It transforms your main function into an async runtime entry point.
async fn main() -> Result<(), Error> {
    let owner = "alisha-reddy";
    let repo = "rust_basics";
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers");
    println!("{}", request_url);
    let client = reqwest::Client::new(); //creates an HTTP client
    let response = client
    .get(&request_url) //creates a GET request
    .header(USER_AGENT, "rust web-api-client demo")
    .send()
    .await?;


    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}