use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub fn get_path_to_exchange_cache(source: &str) -> PathBuf {
    return PathBuf::from(format!(
        r"./.teonite/cache/{}{}.json",
        source,
        get_current_date()
    ));
}

pub fn get_path_to_currency_cache(source: &str) -> PathBuf {
    return PathBuf::from(format!(r"./.teonite/cache/{}.json", source,));
}

fn get_current_date() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%H00%d%m%Y").to_string()
}

pub fn cache_data<T: Serialize>(path: &PathBuf, rates: &T) -> Result<()> {
    fs::create_dir_all(path.parent().unwrap())?; //unwrap since we know we have parent dir
    let serialized: String = serde_json::to_string(rates)?;

    fs::write(&path, serialized)
        .context(format!("Failed to cache exchange rates: '{:?}'", path))?;

    eprintln!("Cached in: '{:?}'", path);
    Ok(())
}

pub fn read_from_cache<T: for<'a> Deserialize<'a>>(path: &PathBuf) -> Option<T> {
    return match fs::read_to_string(path) {
        Ok(contents) => {
            let obj: T = serde_json::from_str(&contents).expect(
                "Failed to parse cached data. Please clear .teonite/cache folder.",
            );
            Some(obj)
        }
        Err(_) => None,
    };
}