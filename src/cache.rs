use std::{fs, path::PathBuf};
use chrono::{DateTime, Utc};
use anyhow::{Context, Result};

use crate::exchange::Rates;

fn get_path_to_cache_file(source: &str) -> PathBuf {
    return PathBuf::from(format!(
        r"./.teonite/cache/{}{}.json",
        source,
        get_current_date()
    ));
}

fn get_current_date() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%H00%d%m%Y").to_string()
}

pub fn cache_exchange_rates(source: &str, rates: &Rates) -> Result<()> {
    let path = get_path_to_cache_file(source);
    fs::create_dir_all(path.parent().unwrap())?; //unwrap since we know we have parent dir
    let serialized: String = serde_json::to_string(rates)?;

    fs::write(&path, serialized)
        .context(format!("Failed to cache exchange rates: '{:?}'", path))?;

    eprintln!("Cached in: '{:?}'", path);
    Ok(())
}

pub fn get_rates_from_cache(source: &str) -> Option<Rates> {
    let path = get_path_to_cache_file(source);
    return match fs::read_to_string(path) {
        Ok(contents) => {
            let rates: Rates = serde_json::from_str(&contents).expect("Failed to parse cached exchange rate file. Please clear .teonite/cache folder.");

            Some(rates)
        }
        Err(_) => None,
    };
}