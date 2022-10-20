extern crate serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Certificate {
    pub issuer_ca_id: u32,
    pub issuer_name: String,
    pub common_name: String,
    pub name_value: String,
    pub id: u64,
    pub serial_number: String,
}

#[derive(Debug)]
pub struct Subdomain {
    pub url: String,
}
