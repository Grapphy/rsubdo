use crate::models::{Certificate, Subdomain};

pub async fn get_subdomains(domain: &str) -> Result<Vec<Subdomain>, Box<dyn std::error::Error>> {
    let certificates: Vec<Certificate> =
        reqwest::get(format!("https://crt.sh/?q={}&output=json", domain))
            .await?
            .json()
            .await?;
    let subdomains: Vec<String> = certificates
        .into_iter()
        .flat_map(|cert| {
            cert.name_value
                .split('\n')
                .map(|dstr| dstr.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|dstr: &String| dstr != domain)
        .filter(|dstr: &String| !dstr.contains('*'))
        .collect();

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|dstr| Subdomain { url: dstr })
        .collect();

    Ok(subdomains)
}
