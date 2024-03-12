This is the exchange API I've used

https://app.freecurrencyapi.com/

API_KEY variable needs to be defined in .env file

Usage can be printed by running cli program with --help flag

`cargo run -- -s PLN -t USD -a 2.253`

`cargo run -- -s PLN --list`

I used nightly compiler since I wanted to try out `Join<Seperator>` trait

To switch to nightly you have to run: `rustup default nightly` and then restart rust-anylyzer