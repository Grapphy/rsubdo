use crate::models::{Subdomain, ThreatminerResults};

// get subdomains frmo threatminer
pub async fn get_threatminer_subdomains(
    domain: &str,
) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let response: ThreatminerResults = reqwest::get(format!(
        "https://api.threatminer.org/v2/domain.php?q={}&rt=5",
        domain
    ))
    .await?
    .json()
    .await?;

    let subdomains: Vec<Subdomain> = response
        .results
        .into_iter()
        .map(|sub| Subdomain { url: sub })
        .collect();

    Ok(subdomains)
}
