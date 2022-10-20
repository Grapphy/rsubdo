use crate::models::Subdomain;

// Gets subdomains from hackertarget.com
pub async fn get_hackertarget_domains(
    domain: &str,
) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "https://api.hackertarget.com/hostsearch/?q={}",
        domain
    ))
    .await?
    .text()
    .await?;

    let subdomains: Vec<String> = response
        .lines()
        .into_iter()
        .map(|sub| sub.split(',').collect::<Vec<&str>>()[0].to_string())
        .collect();

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|sub| Subdomain { url: sub })
        .collect();

    Ok(subdomains)
}
