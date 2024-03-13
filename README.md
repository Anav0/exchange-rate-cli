# Usage 

This is the exchange API I've used

https://app.freecurrencyapi.com/

API_KEY variable needs to be defined in .env file

Usage can be printed by running cli program with --help flag

`cargo run -- -s PLN -t USD,EUR -a 2.30`

`cargo run -- -s PLN --list`

I used nightly compiler since I wanted to try out `Join<Seperator>` trait

To switch to nightly you have to run: `rustup default nightly` and then restart rust-anylyzer

# Things missing:

- Clear old cache files
- Tests
- i18n
- logging of requests