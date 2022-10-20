mod crtsh;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subdomains = crtsh::get_subdomains("facebook.com").await?;
    for sub in subdomains.into_iter() {
        println!("{}", sub.url);
    }

    Ok(())
}
