use std:: fs::read_to_string;
use anyhow::{Result, Context, Ok, anyhow};
use log::{debug, error};
use regex::Regex;
use reqwest::{header::{HeaderMap, HeaderValue}, blocking::Response};
use url::Url;

use crate::config::Config;

use super::{serverless::ServerlessClient, stateful::StatefulClient};


/// Implements various formatters for displaying project data
pub trait ResultFormatting {
    fn compact(&self) -> String;
}

/// a base ESS client handler, capable of serverless and stateful api calls
pub struct ESSClient {
    client: reqwest::blocking::Client,
    auth_req: HeaderMap,
    endpoints: Config
}

impl ESSClient {
    /// create a new handler for the given config
    pub fn new(endpoint: Config) -> Result<Self> {
        let expanded =  shellexpand::tilde(&endpoint.config.key_path);
        let key = read_to_string(expanded.to_string()).context("error reading key path")?;

        let mut headers = HeaderMap::new();
        let key_auth = format!("ApiKey {}", key);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
        headers.insert("Authorization", HeaderValue::from_str(&key_auth)?);

        Ok(ESSClient { client: reqwest::blocking::Client::new(), 
            endpoints: endpoint,
            auth_req: headers})
    }
    /// return a serverless handler for the client
    pub fn serverless(&self) -> Result<ServerlessClient> {
        let cfg = self.endpoints.resolve_serverless();
        let full_path = Url::parse(&cfg.url)?.join(&cfg.base_path)?;
        Ok(ServerlessClient { client: self, 
            base_url: full_path, 
            project: self.endpoints.config.project.clone()})
    }
    /// return a traditional stateful handler for the deployment
    pub fn stateful(&self) -> Result<StatefulClient> {
        let cfg = self.endpoints.resolve_stateful();
        let full_path = Url::parse(&cfg.url)?.join(&cfg.base_path)?;

        Ok(StatefulClient { client: self, base_url: full_path })
    }

    /// Performs a GET request to ESS, checks the result, returns a string
    pub fn get(&self, endpoint: &Url, relative_url: &str) -> Result<String>{
        let full_endpoint = endpoint.join(relative_url)?;
        debug!("GET: {}", full_endpoint);
        let mut res = self.client.get(full_endpoint).headers(self.auth_req.clone()).send()?;
        self.handle_http_error(&mut res)?;
        Ok(res.text()?)
    }

    /// Performs a POST request to ESS, checks the result, returns a string
    pub fn post(&self, endpoint: &Url, relative_url: &str, body: Option<String>) -> Result<String> {
        let full_endpoint = endpoint.join(relative_url)?;
        let mut req = self.client.post(full_endpoint.clone()).headers(self.auth_req.clone());
        if let Some(req_body) = body {
            req = req.body(req_body)
        }
        debug!("POST: {}", full_endpoint);
        let mut res = req.send()?;
        self.handle_http_error(&mut res)?;
        Ok(res.text()?)
    }

    /// Performs a DELETE request to ESS
    pub fn delete(&self, endpoint: &Url, relative_url: &str) -> Result<String> {
        let full_endpoint = endpoint.join(relative_url)?;
        debug!("DELETE: {}", full_endpoint);
        let mut res = self.client.delete(full_endpoint).headers(self.auth_req.clone()).send()?;
        self.handle_http_error(&mut res)?;
        Ok(res.text()?)
    }

    fn handle_http_error(&self, resp: &mut Response) -> Result<()> {
        let status = resp.status();
        if status.is_client_error() || status.is_server_error() {
            let mut buf: Vec<u8> = vec![];
            resp.copy_to(&mut buf)?;
            let string_resp = String::from_utf8_lossy(&buf);
            error!("got error response ({}) from server: \n{}", status, string_resp);
            return Err(anyhow!("got error from server: {}", status));
        }
        Ok(())
    }
}

/// check to see if a user-supplied ID value is valid
pub fn check_id(id: &str) -> Result<()> {
    let re = Regex::new(r"^[a-z0-9-]{32,36}$").unwrap();
    match re.is_match(id) {
        true => Ok(()),
        false => Err(anyhow!("ID '{}' does not appear to be an actual ID value. Did you supply a name value instead?", id))
    }
}