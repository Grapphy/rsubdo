use crate::models::Subdomain;

// get subdomains from omnisint.io
pub async fn get_omnisint_subdomains(
    domain: &str,
) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let response: Vec<String> =
        reqwest::get(format!("https://sonar.omnisint.io/subdomains/{}", domain))
            .await?
            .json()
            .await?;

    let subdomains: Vec<Subdomain> = response
        .into_iter()
        .map(|sub| Subdomain { url: sub })
        .collect();

    Ok(subdomains)
}
