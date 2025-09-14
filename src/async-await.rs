use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{

    let res = reqwest::get("https://jsonplaceholder.typicode.com/posts/1").await?;
    println!("Status: {}", res.status());
    println!("Headers: \n {:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n {}", body);

    Ok(())
}
// cargo run --bin async-await