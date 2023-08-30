use std::{error::Error, time::Duration};

use crates_io_api::{CratesQuery, Crate};

pub async fn search_packages(search_term: &str) -> Result<Vec<Crate>, Box<dyn Error>> {
    let client = crates_io_api::AsyncClient::new("cargo-wizard", Duration::from_millis(100))?;
    let result = client.crates(CratesQuery::builder().search(search_term).build()).await?;
    Ok(result.crates)
}
