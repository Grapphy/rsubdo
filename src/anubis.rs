use crate::models::Subdomain;

// Scrapes subdomains from jonlu.ca anubis
pub async fn get_anubis_subdomains(
    domain: &str,
) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let results: Vec<String> =
        reqwest::get(format!("https://jonlu.ca/anubis/subdomains/{}", domain))
            .await?
            .json()
            .await?;

    let subdomains: Vec<Subdomain> = results
        .into_iter()
        .map(|sub| Subdomain { url: sub })
        .collect();

    Ok(subdomains)
}
