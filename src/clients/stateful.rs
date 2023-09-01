use anyhow::{Result, Context};
use url::Url;
use super::{client::{self, check_id}, deployments::{DeploymentsList, DeploymentGetResponse, DeploymentShutdownResponse}, create_deployment::{DeploymentCreateRequest, DeploymentCreateResponse}};

pub struct StatefulClient<'a>{
    pub client: &'a client::ESSClient,
    pub base_url: Url
}

impl StatefulClient<'_> {
    /// List all deployments
    pub fn list(&self) -> Result<DeploymentsList> {
        let res = self.client.get(&self.base_url, "deployments")?;
        let data: DeploymentsList = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// Get a Deployment
    pub fn get(&self, id: &str) -> Result<DeploymentGetResponse> {
        check_id(id)?;
        let path = format!("deployments/{}", id);
        let res = self.client.get(&self.base_url, &path)?;
        let data: DeploymentGetResponse = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// Shutdown a deployment
    pub fn shutdown(&self, id: &str) -> Result<DeploymentShutdownResponse> {
        check_id(id)?;
        let path = format!("deployments/{}/_shutdown", id);
        let res = self.client.post(&self.base_url, &path, None)?;
        let data: DeploymentShutdownResponse = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// create a deployment from the given config
    pub fn create(&self, dep: DeploymentCreateRequest) -> Result<DeploymentCreateResponse> {
        let path = String::from("deployments");
        let body = serde_json::to_string_pretty(&dep).context("error creating JSON from DeploymentCreateRequest")?;
        let res = self.client.post(&self.base_url, &path, Some(body))?;
        let data: DeploymentCreateResponse = serde_json::from_str(&res).context("error reading response body")?;
        Ok(data)
    }
}