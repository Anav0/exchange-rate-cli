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
        for (_, value) in &self.errors {
            write!(f, "{}\n", value.join(", "))?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rates {
    pub data: HashMap<String, f32>,
}

impl Rates {
    pub fn print_with_info(&self, info: &Currencies) {
        println!("{:<25} {:<4} {}", "Name", "Code", "Rate");
        println!("{:-<42}", "");
        for (currency, rate) in &self.data {
            if let Some(info) = info.data.get(currency) {
                println!("{:<25} {:<4}: {}", info.name, currency, rate);
            } else {
                println!("{:<25} {:<4}: {}", " ", currency, rate);
            }
        }
    }
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
pub struct Currencies {
    pub data: HashMap<String, Currency>,
}

const API_URL: &str = "https://api.freecurrencyapi.com/v1";

pub fn fetch<U: std::fmt::Debug + Display + IntoUrl, T: for<'a> Deserialize<'a>>(
    url: U,
) -> Result<T> {
    dbg!(&url);
    let response: Response = reqwest::blocking::get(url)?;
    if response.status() == 422 {
        let error: ApiError = response.json().expect("Faild to parse API error message");
        bail!("{}", error);
    }
    let obj: T = response.json().with_context(|| {
        format!(
            "Faild to parse requests response as: {}",
            std::any::type_name::<T>()
        )
    })?;

    Ok(obj)
}

pub fn fetch_rates<'a, T>(source: &str, targets: &[T], api_key: &str) -> Result<Rates>
where
    T: AsRef<str>,
    [T]: Join<&'a str, Output = String>,
{
    let joined_codes = targets.join(",");
    let full_url = format!(
        "{API_URL}/latest?apikey={api_key}&currencies={joined_codes}&base_currency={source}"
    );
    let rates: Rates = fetch(full_url)?;
    Ok(rates)
}

pub fn fetch_currencies(api_key: &str) -> Result<Currencies> {
    let full_url = format!("{API_URL}/currencies?apikey={api_key}");
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
