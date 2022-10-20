use itertools::Itertools;

mod alienvault;
mod anubis;
mod crtsh;
mod hackertarget;
mod models;
mod omnisint;
mod threatminer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Not enought arguments: ./main <subdomain>");
        return Ok(());
    }

    let target = args[1].as_str();
    let (alienvault, anubis, crtsh, hackertarget, omnisint, threatminer) = futures::join!(
        alienvault::get_alienvault_subdomains(target),
        anubis::get_anubis_subdomains(target),
        crtsh::get_crt_domains(target),
        hackertarget::get_hackertarget_domains(target),
        omnisint::get_omnisint_subdomains(target),
        threatminer::get_threatminer_subdomains(target)
    );

    let duration = start.elapsed();
    let subdomains: Vec<_> = alienvault
        .iter()
        .flatten()
        .chain(anubis.iter().flatten())
        .chain(crtsh.iter().flatten())
        .chain(hackertarget.iter().flatten())
        .chain(omnisint.iter().flatten())
        .chain(threatminer.iter().flatten())
        .unique_by(|s| &s.url)
        .collect();

    let total = subdomains.len();
    for sub in subdomains.into_iter() {
        println!("{}", sub.url);
    }

    println!("\nScrapped {} domains in {:?}", total, duration);

    Ok(())
}
