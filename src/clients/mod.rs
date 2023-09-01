pub mod serverless;
pub mod stateful;
pub mod client;
pub mod deployments;
pub mod create_deployment;

use anyhow::Result;

use self::serverless::ProjectsList;

pub trait EssHandler {
    fn list(&self) -> Result<ProjectsList>;
}