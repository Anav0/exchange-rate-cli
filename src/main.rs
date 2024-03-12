#![feature(slice_concat_trait)]

use crate::{
    cache::{cache_exchange_rates, get_rates_from_cache},
    exchange::{exchange, fetch_rates, get_all_currency_codes, get_rate, CurrencyCode},
    params::Parameters,
};
use anyhow::{bail, Context, Result};
use dotenv::dotenv;
use std::{env, process::exit};

mod cache;
mod exchange;
mod params;

// Git style help printing
fn print_help() {
    println!("Simple cli currecy converter");
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

fn main() -> Result<()> {
    dotenv().ok();

    let api_key: String = std::env::var("API_KEY").context("API_KEY must be set in .env file")?;

    let params = Parameters::try_from(env::args())?;

    if params.print_help {
        print_help();
        exit(0);
    }

    if params.list_all_rates {
        let all_codes = get_all_currency_codes().context("Failed to get all currency codes")?;
        let codes = all_codes.keys().cloned();
        let rates = fetch_rates(&params.source_currency_code, codes, &api_key)?;
        println!("{}", rates);
        exit(0);
    }

    let rates = get_rates_from_cache(&params.source_currency_code).or_else(|| {
        let codes = vec![params.target_currency_code.clone()];
        fetch_rates(&params.source_currency_code, codes, &api_key)
            .map_err(|e| println!("{}", e))
            .inspect(|r| {
                cache_exchange_rates(&params.source_currency_code, r);
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
            "Faild to find exchange rate for: '{}' - '{}'",
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
