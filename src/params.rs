use std::process::exit;

use anyhow::Context;

use crate::print_help;

// Normalnie użyłbym biblioteki clap do parsowania argumentów ale
// chciałem zaprezentować znajomość From<Args>
#[derive(Debug)]
pub struct Parameters {
    pub source_currency_code: String,
    pub target_currency_code: String,
    pub force_refetch: bool,
    pub amount: f32,
}

//TODO: round to three decimal places
impl TryFrom<std::env::Args> for Parameters {

    fn try_from(mut args: std::env::Args) -> Result<Self, Self::Error> {
        let mut force_refetch = false;
        let mut source_currency_code = String::new();
        let mut target_currency_code = String::new();
        let mut amount: f32 = 0.0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" => {
                    source_currency_code = args
                        .next()
                        .context("Expected valid currency code after -s parameter")?
                }
                "-t" => {
                    target_currency_code = args
                        .next()
                        .context("Expected valid currency code after -t paramter")?
                }
                "-a" => {
                    amount = args
                        .next()
                        .context("Expected valid amount after -a paramter")?
                        .parse()
                        .context("Amount given is not a valid number")?
                }
                "-f" | "--force" => force_refetch = true,
                "-h" | "--help" => {
                    print_help();
                    exit(0)
                }
                _ => {}
            }
        }

        Ok(Self {
            source_currency_code,
            target_currency_code,
            amount,
            force_refetch,
        })
    }
    
    type Error = anyhow::Error;
    
}
