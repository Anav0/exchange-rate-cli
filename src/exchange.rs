use std::{
    collections::HashMap,
    fmt::{self, Display},
    slice::Join,
};

use anyhow::{bail, Context, Result};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
    errors: HashMap<String, Vec<String>>,
    info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rates {
    data: HashMap<String, f32>,
}

impl Display for Rates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (currency, rate) in &self.data {
            writeln!(f, "{}: {}", currency, rate)?;
        }
        Ok(())
    }
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

pub fn fetch_rates<'a, C, T>(source: &str, targets: C, api_key: &str) -> Result<Rates>
where
    C: IntoIterator<Item = T>,
    T: AsRef<str>,
    [T]: Join<&'a str, Output = String>,
{
    let url = "https://api.freecurrencyapi.com/v1/latest";

    let full_url = format!(
        "{}?apikey={}&currencies={}&base_currency={}",
        url,
        api_key,
        targets.into_iter().collect::<Vec<T>>().join(","),
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

pub fn exchange(target: &str, amount: f32, rates: &Rates) -> Option<f32> {
    if !rates.data.contains_key(target) {
        return None;
    }

    let rate = rates.data.get(target)?;

    Some(amount * rate)
}

pub fn get_rate(rates: &Rates, currency_code: &str) -> Option<f32> {
    return rates.data.get(currency_code).copied();
}

pub type CurrencyCode = String; // [char; 3]

#[derive(Debug, Deserialize, Serialize)]
pub struct Currency {
    pub name: String,
    pub decimal_digits: u8,
    pub rounding: f32,
    pub code: CurrencyCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyInfo {
    pub data: HashMap<String, Currency>
}

pub fn get_all_currency_codes(source: &str, api_key: &str) -> Result<CurrencyInfo> {
    let full_url= format!("https://api.freecurrencyapi.com/v1/currencies?apikey={api_key}&base_currency={source}");
    let response: Response = reqwest::blocking::get(full_url)?;
    let currency_info: CurrencyInfo = response.json().context("Faild to parse currency information")?;
    Ok(currency_info)
}
