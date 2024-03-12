use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs,
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
    T: AsRef<str>,
    [T]: Join<&'a str, Output = String>,
    C: IntoIterator<Item = T>,
{
    let url = "https://api.freecurrencyapi.com/v1/latest";

    let y : HashMap<String, u32>;
    // let xx = y.keys().cloned().collect::<Vec<String>>().join(",");
    // let x = vec!["A", "b"].join(",");

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

#[derive(Debug, Deserialize)]
pub struct Currency {
    pub symbol: String,
    pub name: String,
    pub symbol_native: String,
    pub decimal_digits: u8,
    pub rounding: f32,
    pub code: CurrencyCode,
    pub name_plural: String,
}

pub fn get_all_currency_codes() -> Result<HashMap<CurrencyCode, Currency>> {
    let contents = fs::read_to_string(r"resources\currrencies.json")?;
    let currencies_map: HashMap<CurrencyCode, Currency> =
        serde_json::from_str(&contents).context("Failed to parse currencies.json file")?;

    Ok(currencies_map)
}
