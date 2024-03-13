use anyhow::{bail, Result};

use crate::{
    cache::{get_path_to_currencies_cache, read_from_cache},
    exchange::Currencies,
    params::Parameters,
};

/*NOTE: If we had a complex structure to validate. I would use this pattern:
    for validator in validators {
         validator.validate(&params)?;
    }
*/

pub fn validate(params: &Parameters) -> Result<()> {
    let path = get_path_to_currencies_cache();

    let mut invalid_target_currencies: Vec<&str> = Vec::with_capacity(180);

    if let Some(info) = read_from_cache::<Currencies>(&path) {
        if !info.data.contains_key(&params.source_currency_code) {
            bail!(
                "'{}' is not a valid currency code",
                &params.source_currency_code
            );
        }
        for code in &params.target_currency_code {
            if !info.data.contains_key(code) {
                invalid_target_currencies.push(code);
            }
        }
    }

    if invalid_target_currencies.len() > 0 {
        bail!(
            "This passed target currencies are invalid, or cannot be exchanged for '{}': '{}'",
            &params.source_currency_code,
            invalid_target_currencies.join(", ")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{params::Parameters, validation::validate};

    #[test]
    fn check_source_currency_code() {
        let mut params: Parameters = Parameters::new();
        params.source_currency_code = String::from("AAA");
        let result = validate(&params);
        assert!(result.is_err());
        let e = result.unwrap_err();
        let root_cause = e.root_cause();
        assert_eq!(
            format!("{}", root_cause),
            "'AAA' is not a valid currency code"
        );
    }
}
