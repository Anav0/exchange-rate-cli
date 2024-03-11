use crate::{
    cache::{cache_exchange_rates, get_rates_from_cache},
    exchange::{exchange, fetch_rates},
    params::Parameters,
};
use anyhow::{bail, Context, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

mod cache;
mod exchange;
mod params;

#[derive(Debug, Deserialize, Serialize)]
struct Rates {
    data: HashMap<String, f32>,
}

// Git style help printing
fn print_help() {
    println!("Simple cli currecy converter");
    println!("Usage:");
    println!("\t./teonite -s PLN -t USD -a 43.123 --live");
    println!("Parameters:");
    println!("{:<12}{}", "-s", "source currency code e.g. EUR");
    println!("{:<12}{}", "-t", "target currency code e.g. USD");
    println!(
        "{:<12}{}",
        "-a", "amount to convert from source currency to target currency"
    );
    println!("{:<12}{}", "-l, --live", "live feedback");
    println!("{:<12}{}", "-f, --force", "fetch data each time");
    println!("{:<12}{}", "--tutorial", "Interactive tutorial");
}

fn main() -> Result<()> {
    dotenv().ok();

    let api_key: String = std::env::var("API_KEY").context("API_KEY must be set in .env file")?;

    let params = Parameters::from(env::args());

    let rates = get_rates_from_cache(&params.source_currency_code).or_else(|| {
        fetch_rates(
            &params.source_currency_code,
            vec![&params.target_currency_code],
            &api_key,
        ).map_err(|e| println!("{}", e)).ok()
    });

    if rates.is_none() {
        return Ok(())
    }

    let after_exchange = exchange(
        &params.source_currency_code,
        &params.target_currency_code,
        params.amount,
        &rates.unwrap(),
    );

    if after_exchange.is_none() {
        bail!(
            "Faild to find exchange rate for: '{}' - '{}'",
            &params.source_currency_code,
            &params.target_currency_code
        );
    }

    println!(
        "{}{} is equal to {}{}",
        params.amount,
        params.source_currency_code,
        after_exchange.unwrap(),
        params.target_currency_code
    );

    Ok(())
}
