use std::{env, process::exit};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// Normalnie użyłbym biblioteki clap do parsowania argumentów ale
// chciałem zaprezentować znajomość Fram<Args>
#[derive(Debug)]
struct Parameters {
    source_currency_code: String,
    target_currency_code: String,
    live_feedback: bool,
    amount: f32, // u32 zamiast f32 aby nie nadziać się na probelmy z utratą precyzji. Zamiast 2.33zł mamy więc 233
}

//TODO: round to three decimal places

impl From<std::env::Args> for Parameters {
    fn from(mut args: std::env::Args) -> Self {
        let mut live_feedback = false;
        let mut source_currency_code = String::new();
        let mut target_currency_code = String::new();
        let mut amount: f32 = 0.0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-l" | "--live" => live_feedback = true,
                "-s" => source_currency_code = args.next().expect("Expected valid currency code after -s parameter"),
                "-t" => target_currency_code = args.next().expect("Expected valid currency code after -t paramter"),
                "-a" => amount = args.next().expect("Expected valid amount after -a paramter").parse().expect("Amount given is not a valid number"),
                "-h" | "--help" => { print_help(); exit(0) }, 
                "--tutorial" => { tutorial(); exit(0) }, 
                _=> {},
            }
        }

        Self {
            source_currency_code,
            target_currency_code,
            amount,
            live_feedback
        }
    }
}

fn tutorial() {

}

// Git repo style help printing
fn print_help() {
    println!("Simple cli currecy converter");
    println!("Usage:");
    println!("\t./teonite -s PLN -t USD -a 43.123 --live");
    println!("Parameters:");
    println!("{:<12}{}", "-s", "source currency code e.g. EUR");
    println!("{:<12}{}", "-t", "target currency code e.g. USD");
    println!("{:<12}{}", "-a", "amount to convert from source currency to target currency");
    println!("{:<12}{}", "-l, --live", "live feedback");
    println!("{:<12}{}", "--tutorial", "Interactive tutorial");
}

fn main() {
    let params = Parameters::from(env::args());

    println!("{:?}", params);
}
