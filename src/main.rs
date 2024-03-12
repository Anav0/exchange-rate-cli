#![feature(slice_concat_trait)]

use crate::{
    cache::{cache_data, get_path_to_currency_cache, get_path_to_exchange_cache, read_from_cache},
    exchange::{exchange, fetch_currency_info, fetch_rates, get_rate},
    params::Parameters,
};
use anyhow::{bail, Context, Result};
use dotenv::dotenv;
use std::env;

mod cache;
mod exchange;
mod params;

// Git style help printing
fn print_help() {
    println!("Simple cli currency converter");
    println!("Usage:");
    println!("\t./teonite -s PLN -t USD -a 43.123");
    println!("Parameters:");
    println!("{:<12}{}", "-s", "source currency code e.g. EUR");
    println!("{:<12}{}", "-t", "target currency code e.g. USD");
    println!(
        "{:<12}{}",
        "-a", "amount to convert from source currency to target currency"
    );
    println!("{:<12}{}", "-f, --force", "fetch data each time");
}

fn print_all_exchange_rates(params: &Parameters, api_key: &str) -> Result<()> {
    let path = get_path_to_currency_cache(&params.source_currency_code);
    let currency_info = read_from_cache(&path).or_else(|| {
        fetch_currency_info(&params.source_currency_code, &api_key)
            .map_err(|e| println!("{}", e))
            .inspect(|info| {
                let _ = cache_data(&path, info);
            })
            .ok()
    });
    if currency_info.is_none() {
        bail!("Failed to fetch currency information");
    }
    let currency_info = currency_info.unwrap();
    let codes = currency_info.data.keys().cloned();
    let rates = fetch_rates(&params.source_currency_code, codes, &api_key)?;
    cache_data(&path, &currency_info)?;
    println!("{}", rates);
    Ok(())
}

fn print_single_exchange_rate(params: &Parameters, api_key: &str) -> Result<()> {
    let path = get_path_to_exchange_cache(&params.source_currency_code);
    let rates = read_from_cache(&path).or_else(|| {
        let codes = vec![params.target_currency_code.clone()];
        fetch_rates(&params.source_currency_code, codes, &api_key)
            .map_err(|e| println!("{}", e))
            .inspect(|r| {
                let _ = cache_data(&path, r);
            })
            .ok()
    });
    if rates.is_none() {
        return Ok(());
    }
    let rates = rates.unwrap(); //At this point we know we have rates
    let after_exchange = exchange(&params.target_currency_code, params.amount, &rates);
    if after_exchange.is_none() {
        bail!(
            "Cannot exchange: '{}' - '{}'",
            &params.source_currency_code,
            &params.target_currency_code
        );
    }
    println!(
        "{} {} is equal to {} {} (rate: {})",
        params.amount,
        params.source_currency_code,
        after_exchange.unwrap(),
        params.target_currency_code,
        get_rate(&rates, &params.target_currency_code).unwrap(),
    );
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();

    let api_key: String = std::env::var("API_KEY").context("API_KEY must be set in .env file")?;

    let params = Parameters::try_from(env::args())?;

    if params.print_help {
        print_help();
        return Ok(());
    }

    if params.list_all_rates {
        print_all_exchange_rates(&params, &api_key)?;
        return Ok(());
    }

    print_single_exchange_rate(&params, &api_key)?;

    Ok(())
}
