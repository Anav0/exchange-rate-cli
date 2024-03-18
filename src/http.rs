use std::fmt::Display;

use anyhow::{bail, Context};
use reqwest::blocking::{Client, Response};
use reqwest::{Certificate, IntoUrl};
use serde::Deserialize;

pub struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        let inner_client = reqwest::blocking::ClientBuilder::new()
            .build()
            .expect("Failed to construct HttpClient that uses root TLS certs");
        
        Self { client: inner_client }
    }
}

impl HttpClient {
    pub fn new(client: Client) ->Self{
        Self {
            client
        }
    }

    pub fn fetch<U: std::fmt::Debug + Display + IntoUrl, T: for<'a> Deserialize<'a>, E: Display + for<'a> Deserialize<'a>>(
        &self,
        url: U,
    ) -> anyhow::Result<T> {
        let response = self.client.get(url).send()?;
        if response.status() == 422 {
            let error: E = response.json().expect("Faild to parse API error message");
            bail!("{}", error);
        }
        let obj: T = response.json().with_context(|| {
            format!(
                "Faild to parse requests response as: {}",
                std::any::type_name::<T>()
            )
        })?;
    
        Ok(obj)
    }
}