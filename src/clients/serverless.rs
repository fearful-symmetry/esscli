
use std::{thread, time};

use log::debug;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Ok}; 

use super::client::{self, ResultFormatting, check_id};

/// Client for serverless ESS.
/// Normally instatiated via the EssClient.serverless() call.
pub struct ServerlessClient<'a>{
    pub project: String,
    pub client: &'a client::ESSClient,
    pub base_url: url::Url,
}

/// List of all serverless Projects
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsList {
    pub items: Vec<Project>
}

impl ResultFormatting for ProjectsList {
    fn compact(&self) -> String {
        let mut acc = String::new();
        for item in &self.items{
            acc = format!("{}{}\n", acc, item.compact());
        };
        acc
    }
}

/// Data on an individual serverless project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub alias: String,
    pub id: String,
    pub metadata: ProjectMetadata,
    pub name: String,
    pub region_id: String, 
    pub cloud_id: String,
    pub endpoints: ProjectEndpoints,
    #[serde(alias = "type")]
    pub project_type: String,
}

impl ResultFormatting for Project{
    fn compact(&self) -> String {
        format!("{}, {}, {}", self.name, self.id, self.metadata.created_at)
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

/// Metadata for a serverless project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    pub created_at: String,
    pub created_by: String,
    pub organization_id: String
}

/// Stack endpoints for a project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectEndpoints {
    pub apm: String,
    pub elasticsearch: String,
    pub kibana: String
}

/// user/pass combination for a project.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCredentials {
    pub username: String,
    pub password: String
}

impl ResultFormatting for ProjectCredentials {
    fn compact(&self) -> String {
        format!("{},{}", self.username, self.password)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStatus {
    pub phase: String
}

impl ResultFormatting for ProjectStatus {
    fn compact(&self) -> String {
        self.phase.clone()
    }
}

/// Provides data on a cloud region
#[derive(Serialize, Deserialize, Debug)]
pub struct Region{
    pub csp: String,
    pub csp_region: String,
    pub id: String,
    pub name: String
}

impl ResultFormatting for Region{
    fn compact(&self) -> String {
        format!("{}: {} ({})", self.id, self.name, self.csp)
    }
}

/// Overrides for components that can be set during project creation.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<ApplicationOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kibana: Option<ApplicationOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<ApplicationOverride>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationOverride {
    pub docker_image: String
}

/// The body of a project create request 
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProject {
    pub name: String,
    pub region_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ProjectOverrides>
}

impl ServerlessClient<'_>{
    /// list all projects on the system
    pub fn list(&self) -> Result<ProjectsList> {
        let path = format!("projects/{}", self.project);
        let res = self.client.get(&self.base_url, &path)?;
        let data:ProjectsList = serde_json::from_str(&res)?;
        Ok(data)
    }
    /// Get a project
    pub fn get(&self, id: &str) -> Result<Project> {
        check_id(id)?;
        let path = format!("projects/{}/{}", self.project, id);
        let res = self.client.get(&self.base_url, &path)?;
        let data: Project = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// reset credentials for a project
    pub fn reset_credentials(&self, id: &str) -> Result<ProjectCredentials> {
        check_id(id)?;
        let path = format!("projects/{}/{}/_reset-credentials", self.project, id);
        let res = self.client.post(&self.base_url, &path, None)?;
        let data: ProjectCredentials = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// get the status of a project
    pub fn status(&self, id: &str) -> Result<ProjectStatus> {
        check_id(id)?;
        let path = format!("projects/{}/{}/status", self.project, id);
        let res = self.client.get(&self.base_url, &path)?;
        let data: ProjectStatus = serde_json::from_str(&res)?;
        Ok(data)
    }

    /// delete a project
    pub fn delete(&self, id: &str) -> Result<()> {
        check_id(id)?;
        let path = format!("projects/{}/{}", self.project, id);
        self.client.delete(&self.base_url, &path)?;
        Ok(())
    }

    /// create a new project. if "wait" is provided, the method will block until
    /// the project is ready to be connected to.
    pub fn create(&self, proj: CreateProject, wait: bool) -> Result<Project> {
        let path = format!("projects/{}", self.project);
        let body = serde_json::to_string(&proj)?;
        let res = self.client.post(&self.base_url, &path, Some(body))?;
        let mut data: Project = serde_json::from_str(&res)?;

        if wait{
            debug!("waiting for endpoints...");
            loop {
                if !data.endpoints.apm.is_empty() && !data.endpoints.elasticsearch.is_empty() && !data.endpoints.kibana.is_empty(){
                    break;
                }
                let resp = self.get(&data.id)?;
                data = resp;
                thread::sleep(time::Duration::from_millis(300))
            }
        }
        Ok(data)
    }

    /// List all available regions
    pub fn regions(&self) -> Result<Vec<Region>> {
        let res = self.client.get(&self.base_url, "regions")?;
        let data: Vec<Region> = serde_json::from_str(&res)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, clients::client::ESSClient};

    use super::CreateProject;


    fn setup_tests() -> ESSClient {
        let cfg = Config{
            statefull_override: None,
            serverless_override: None,
            config: crate::config::UserConfig {
                default_deployment: "~/.config/ess/api_key".to_string(),
                 project: "observability".to_string(),
                 key_path: "~/.config/ess/api_key.txt".to_string(), 
                },
                defaults: crate::config::TypeConfig {
                     url: "https://global.qa.cld.elstc.co".to_string(),
                     base_path: "/api/v1/serverless/".to_string() 
                ,}
        };

        ESSClient::new(cfg).unwrap()
    }

    #[test]
    fn test_list(){
        let client = setup_tests();
        let _res = client.serverless().unwrap().list().unwrap();
    }

    #[test]
    fn test_create_delete(){
        let client = setup_tests();

        let req = CreateProject{
            name: String::from("test_create_delete_cli"),
            region_id: String::from("aws-eu-west-1"),
            overrides: None
        };
        let resp = client.serverless().unwrap().create(req, false).unwrap();
        assert!(resp.id.len() > 0)
    }
}