//! # Send JSON encoded HTTP requests

use super::{Error, MethodParams};
use hyper::Url;
use hyper::client::IntoUrl;
use jsonrpc_core::{self, Value};
use jsonrpc_core::futures::{BoxFuture, Future};
use reqwest::Client;

pub struct AsyncWrapper {
    pub url: Url,
}

impl AsyncWrapper {
    pub fn new<U: IntoUrl>(url: U) -> AsyncWrapper {
        AsyncWrapper { url: url.into_url().expect("Expect to encode request url") }
    }

    /// Send and JSON RPC HTTP request
    pub fn request(&self, params: &MethodParams) -> BoxFuture<Value, jsonrpc_core::Error> {
        match self.send(params) {
            Ok(res) => ::futures::finished(res).boxed(),
            Err(err) => {
                error!("{}", err);
                ::futures::failed(jsonrpc_core::Error::from(err)).boxed()
            }
        }
    }

    fn send(&self, params: &MethodParams) -> Result<Value, Error> {
        let client = Client::new()?;
        let mut res = client.post(self.url.clone()).json(params).send()?;
        let json: Value = res.json()?;
        Ok(json["result"].clone())
    }
}
