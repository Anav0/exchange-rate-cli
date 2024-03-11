use std::{collections::HashMap, fmt::{self, Display}};

use reqwest::blocking::Response;
use anyhow::{bail, Error, Result};
use serde::Deserialize;

use crate::Rates;

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
    errors: HashMap<String, Vec<String>>,
    info: String,
}

impl Display for ApiError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiError: {}\n", self.message)?;
        write!(f, "Errors:\n")?;
        for (key, value) in &self.errors {
            write!(f, " - {}: {}\n", key, value.join(", "))?;
        }
        Ok(())
    }
}

pub fn fetch_rates(source: &str, targets: Vec<&str>, api_key: &str) -> Result<Rates> {
    let url = "https://api.freecurrencyapi.com/v1/latest";
    let full_url = format!(
        "{}?apikey={}&currencies={}&base_currency={}",
        url,
        api_key,
        targets.join(","),
        source
    );
    //logging
    eprintln!("{}", full_url);
    let response: Response = reqwest::blocking::get(full_url)?;

    if response.status() == 422 {
        let error: ApiError = response.json().expect("Faild to parse API error message");
        bail!("{}", error);
    }

    let rates: Rates = response
        .json()
        .expect("Failed to parse json data recived from exchange rate service");
    Ok(rates)
}

pub fn exchange(source: &str, target: &str, amount: f32, rates: &Rates) -> Option<f32> {
    if !rates.data.contains_key(target) {
        return None;
    }

    let rate = rates.data.get(target)?;

    Some(amount * rate)
}