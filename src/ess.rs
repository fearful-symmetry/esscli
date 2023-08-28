use std::{path::PathBuf, fs::read_to_string};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Ok}; 
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};
use url::Url;


pub trait EssHandler {
    fn list(&self) -> Result<Projects>;
}

pub struct ServerlessClient{
    client: Client,
    //key: String,
    project: String,
    auth_req: HeaderMap,
    endpoint: url::Url
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    pub items: Vec<Project>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub alias: String,
    pub id: String,
    pub metadata: ProjectMetadata,
    pub name: String,
    pub region_id: String, 
    pub endpoints: ProjectEndpoints,
    #[serde(alias = "type")]
    pub project_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMetadata {
    pub created_at: String,
    pub created_by: String,
    pub organization_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectEndpoints {
    pub apm: String,
    pub elasticsearch: String,
    pub kibana: String
}

impl ServerlessClient{
    pub fn new(key_path: PathBuf, project: String, endpoint: &str) -> Result<Self> {
        let key = read_to_string(key_path)?;

        let mut headers = HeaderMap::new();
        let key_auth = format!("ApiKey {}", key);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
        headers.insert("Authorization", HeaderValue::from_str(&key_auth)?);
        return Ok(ServerlessClient { client: Client::new(), 
            //key: key.clone(), 
            project: project,
            endpoint: Url::parse(&endpoint)?,
            auth_req: headers});
    }

    pub fn list(&self) -> Result<Projects> {
        let endpoint = self.endpoint.join("/api/v1/serverless/projects/")?.join(&self.project)?;
        let res = self.client.get(endpoint).headers(self.auth_req.clone()).send()?;
        res.error_for_status_ref()?;

        let data:Projects = serde_json::from_str(&res.text()?)?;
        Ok(data)
    }
}

impl EssHandler for ServerlessClient {
    fn list(&self) -> Result<Projects>{
        let endpoint = self.endpoint.join("/api/v1/serverless/projects/")?.join(&self.project)?;
        let res = self.client.get(endpoint).headers(self.auth_req.clone()).send()?;
        res.error_for_status_ref()?;

        let data:Projects = serde_json::from_str(&res.text()?)?;
        Ok(data)
    }
}