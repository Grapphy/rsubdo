use crate::models::{AlientVaultDNS, Subdomain};

// Scrape subdomains from alienvault
pub async fn get_alienvault_subdomains(
    domain: &str,
) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let results: AlientVaultDNS = reqwest::get(format!(
        "https://otx.alienvault.com/api/v1/indicators/domain/{}/passive_dns",
        domain
    ))
    .await?
    .json()
    .await?;

    let subdomains: Vec<Subdomain> = results
        .passive_dns
        .into_iter()
        .filter(|sub| sub.hostname.is_some())
        .map(|sub| Subdomain {
            url: sub.hostname.unwrap(),
        })
        .collect();

    Ok(subdomains)
}
