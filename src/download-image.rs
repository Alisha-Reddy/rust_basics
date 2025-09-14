use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let temp_dir = Builder::new().prefix("example").tempdir()?; //creates a temp directory with a name like exampleXXXX. `.tempdir()?`` → actually creates it.
    let target = "https://imgs.search.brave.com/mzET9YOxJUd_4ZErPJEhvX-kltYUsE5etbf_esXs3rc/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9pLnBp/bmltZy5jb20vb3Jp/Z2luYWxzL2Q5LzIx/LzYwL2Q5MjE2MGRh/ODZhNTQ2Mjg5OTc4/YTRkNTg5ZTQzNGJm/LmpwZw";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty(){None} else {Some(name)})
        .unwrap_or("tmp.bin");

        println!("File to download: {}", fname);
        let fname = temp_dir.path().join(fname);
        println!("Will be located under {}", fname.display());
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest);
    Ok(())
}