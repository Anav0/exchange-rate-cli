use std::{
    collections::HashMap,
    fmt::{self, Display},
    slice::Join,
};

use anyhow::{bail, Context, Result};
use reqwest::{blocking::Response, IntoUrl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
    errors: HashMap<String, Vec<String>>,
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
pub type CurrencyCode = String; // Could be [char; 3]?

#[derive(Debug, Deserialize, Serialize)]
pub struct Currency {
    pub name: String,
    pub decimal_digits: u8,
    pub rounding: f32,
    pub code: CurrencyCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyInfo {
    pub data: HashMap<String, Currency>,
}

const API_URL: &str = "https://api.freecurrencyapi.com/v1/latest";

pub fn fetch<U: IntoUrl, T: for<'a> Deserialize<'a>>(url: U) -> Result<T> {
    let response: Response = reqwest::blocking::get(url)?;
    if response.status() == 422 {
        let error: ApiError = response.json().expect("Faild to parse API error message");
        bail!("{}", error);
    }
    let obj: T = response
        .json()
        .context("Faild to parse requests payload as: {T}")?;

    Ok(obj)
}

pub fn fetch_rates<'a, C, T>(source: &str, targets: C, api_key: &str) -> Result<Rates>
where
    C: IntoIterator<Item = T>,
    T: AsRef<str>,
    [T]: Join<&'a str, Output = String>,
{
    let joined_codes = targets.into_iter().collect::<Vec<T>>().join(",");
    let full_url =
        format!("{API_URL}?apikey={api_key}&currencies={joined_codes}&base_currency={source}");

    let rates: Rates = fetch(full_url)?;
    Ok(rates)
}

pub fn fetch_currency_info(source: &str, api_key: &str) -> Result<CurrencyInfo> {
    let full_url = format!("{API_URL}currencies?apikey={api_key}&base_currency={source}");
    let currency_info = fetch(full_url)?;
    Ok(currency_info)
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
