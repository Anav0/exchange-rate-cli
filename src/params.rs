use std::process::exit;

use crate::print_help;

// Normalnie użyłbym biblioteki clap do parsowania argumentów ale
// chciałem zaprezentować znajomość Fram<Args>
#[derive(Debug)]
pub struct Parameters {
    pub source_currency_code: String,
    pub target_currency_code: String,
    pub live_feedback: bool,
    pub force_refetch: bool,
    pub amount: f32, // u32 zamiast f32 aby nie nadziać się na probelmy z utratą precyzji. Zamiast 2.33zł mamy więc 233
}

//TODO: round to three decimal places
impl From<std::env::Args> for Parameters {
    fn from(mut args: std::env::Args) -> Self {
        let mut live_feedback = false;
        let mut force_refetch = false;
        let mut source_currency_code = String::new();
        let mut target_currency_code = String::new();
        let mut amount: f32 = 0.0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-l" | "--live" => live_feedback = true,
                "-s" => {
                    source_currency_code = args
                        .next()
                        .expect("Expected valid currency code after -s parameter")
                }
                "-t" => {
                    target_currency_code = args
                        .next()
                        .expect("Expected valid currency code after -t paramter")
                }
                "-a" => {
                    amount = args
                        .next()
                        .expect("Expected valid amount after -a paramter")
                        .parse()
                        .expect("Amount given is not a valid number")
                }
                "-f" | "--force" => force_refetch = true,
                "-h" | "--help" => {
                    print_help();
                    exit(0)
                }
                "--tutorial" => {
                    exit(0)
                }
                _ => {}
            }
        }

        Self {
            source_currency_code,
            target_currency_code,
            amount,
            force_refetch,
            live_feedback,
        }
    }
}
