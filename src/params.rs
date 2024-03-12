use anyhow::Context;

// Normalnie użyłbym biblioteki clap do parsowania argumentów ale
// chciałem zaprezentować znajomość TryFrom<Args>
#[derive(Debug)]
pub struct Parameters {
    pub source_currency_code: String,
    pub target_currency_code: Vec<String>,
    pub amount: f32,
    pub list_all_rates: bool,
    pub force_refetch: bool,
    pub print_help: bool,
}

impl TryFrom<std::env::Args> for Parameters {
    fn try_from(mut args: std::env::Args) -> Result<Self, Self::Error> {
        let mut force_refetch = false;
        let mut list_all_rates = false;
        let mut print_help = false;

        let mut source_currency_code = String::new();
        let mut target_currency_code = vec![];
        let mut amount: f32 = 0.0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" => {
                    source_currency_code = args
                        .next()
                        .context("Expected valid currency code after -s parameter")?
                }
                "-t" => {
                    let target_currency_str = args
                        .next()
                        .context("Expected valid currency code after -t paramter")?;

                    target_currency_code = target_currency_str
                        .split(",")
                        .map(|v| v.to_string())
                        .collect();
                }
                "-a" => {
                    amount = args
                        .next()
                        .context("Expected valid amount after -a paramter")?
                        .parse()
                        .context("Amount given is not a valid number")?;

                    if amount < 0. {
                        amount *= -1.;
                    }
                    //TODO: adjust rounding based on currencies rounding specification
                    amount = (amount * 1000.0).round() / 1000.0;
                }
                "-f" | "--force" => force_refetch = true,
                "-h" | "--help" => print_help = true,
                "--list" | "-l" => list_all_rates = true,
                _ => {}
            }
        }

        Ok(Self {
            source_currency_code,
            target_currency_code,
            amount,
            force_refetch,
            list_all_rates,
            print_help,
        })
    }

    type Error = anyhow::Error;
}
