#![feature(slice_concat_trait)]

use crate::{
    cache::{
        cache_data, get_path_to_currencies_cache, get_path_to_exchange_cache, read_from_cache,
    },
    exchange::{exchange, fetch_currencies, fetch_rates, get_rate, Rates},
    params::Parameters,
};
use anyhow::{bail, Context, Result};
use cache::{fresh_currency_info_exists_in_cache, update_currency_info_cache};
use dotenv::dotenv;
use http::HttpClient;
use std::env;
use validation::validate;

mod http;
mod cache;
mod exchange;
mod params;
mod validation;

// Git style help printing
fn print_help() {
    println!("Simple cli currency converter");
    println!("Usage:");
    println!("\t./teonite -s PLN -t USD,EUR -a 12.123");
    println!("Parameters:");
    println!("{:<12}{}", "-s", "source currency code e.g. EUR");
    println!("{:<12}{}", "-t", "target currencies code e.g. USD,PLN,EUR");
    println!(
        "{:<12}{}",
        "-a", "amount to convert from source currency to target currency"
    );
    println!("{:<12}{}", "-f, --force", "fetch data each time");
}

fn print_all_exchange_rates(client: &HttpClient, params: &Parameters, api_key: &str) -> Result<()> {
    let path = get_path_to_currencies_cache();
    let currency_info = read_from_cache(&path)
        .and_then(|v| {
            if params.force_refetch {
                return None;
            }
            Some(v)
        })
        .or_else(|| {
            fetch_currencies(client, &api_key)
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
    cache_data(&path, &currency_info)?;

    let path = get_path_to_exchange_cache(&params.source_currency_code);
    let codes: Vec<String> = currency_info.data.keys().cloned().collect();
    let rates = fetch_rates(client, &params.source_currency_code, &codes, &api_key)?;
    cache_data(&path, &rates)?;
    println!("Source currency: '{}'", &params.source_currency_code);
    rates.print_with_info(&currency_info);
    Ok(())
}

//TODO: change String to AsRef
fn update_exchange_rate_cache(
    client: &HttpClient,
    source: &str,
    targets: &Vec<String>,
    api_key: &str,
    mut cached_rates: Rates,
) -> Option<Rates> {
    let mut not_cached_target_exists = false;
    for target in targets {
        if !cached_rates.data.contains_key(target) {
            not_cached_target_exists = true;
            break;
        }
    }

    if !not_cached_target_exists {
        return Some(cached_rates);
    }

    let fetched_rate = fetch_rates(client, &source, targets, &api_key).ok()?;

    for (code, rate) in fetched_rate.data {
        cached_rates.data.insert(code, rate);
    }

    let path = get_path_to_exchange_cache(source);
    let _ = cache_data(&path, &cached_rates);
    Some(cached_rates)
}

fn print_selected_exchange_rate(client: &HttpClient, params: &Parameters, api_key: &str) -> Result<()> {
    let path = get_path_to_exchange_cache(&params.source_currency_code);
    let rates: Option<Rates> = read_from_cache(&path)
        .and_then(|v| {
            update_exchange_rate_cache(
                client,
                &params.source_currency_code,
                &params.target_currency_code,
                &api_key,
                v,
            )
        })
        .or_else(|| {
            fetch_rates(
                client,
                &params.source_currency_code,
                &params.target_currency_code,
                &api_key,
            )
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

    for target in &params.target_currency_code {
        let after_exchange = exchange(&target, params.amount, &rates);
        if after_exchange.is_none() {
            bail!(
                "Cannot exchange: '{}' - '{}'",
                &params.source_currency_code,
                target
            );
        }
        println!(
            "{:.2} {} is equal to {:.2} {} (rate: {})",
            params.amount,
            params.source_currency_code,
            after_exchange.unwrap(),
            target,
            get_rate(&rates, &target).unwrap(),
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    

    let client = HttpClient::default();

    dotenv().ok();
    
    let params = Parameters::try_from(env::args())?;

    if params.print_help {
        print_help();
        return Ok(());
    }

    let api_key: String = std::env::var("API_KEY").context("API_KEY enviroment variable must be set")?;

    if api_key == "" {
        bail!("Invalid API key, please provide valid API_KEY enviroment variable");
    }

    if !fresh_currency_info_exists_in_cache() || params.force_refetch {
        update_currency_info_cache(&client, &api_key)?;
    }

    validate(&params)?;

    if params.list_all_rates {
        print_all_exchange_rates(&client, &params, &api_key)?;
        return Ok(());
    }

    print_selected_exchange_rate(&client, &params, &api_key)?;

    Ok(())
}
