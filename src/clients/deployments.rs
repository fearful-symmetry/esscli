use serde::{Serialize, Deserialize};

use super::client::ResultFormatting;

// ***********************************************************************
// structs for deployment listing/getting

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentsList {
    /// A list of deployments
    pub deployments: Vec<DeploymentsListingData>,
}

impl ResultFormatting for DeploymentsList {
    fn compact(&self) -> String {
        let mut acc = String::new();
        for item in &self.deployments{
            acc = format!("{}{}\n", acc, item.compact());
        };
        acc
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentsListingData {
    /// The id of this deployment
    pub id: String,
    /// List of resources in this deployment
    pub resources: Vec<DeploymentResource>,
    /// The name of this deployment
    pub name: String,
}

impl ResultFormatting for DeploymentsListingData {
    fn compact(&self) -> String {
        format!("{}, {}", self.name, self.id)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentGetResponse {
    /// The name of this deployment
    pub name: String,
    pub settings: Option<DeploymentSettings>,
    /// Whether the deployment is overall healthy or not (one or more of the resource info subsections will have healthy: false)
    pub healthy: bool,
    /// A user-defined deployment alias for user-friendly resource URLs
    pub alias: Option<String>,
    pub observability: Option<DeploymentObservability>,
    /// A randomly-generated id of this Deployment
    pub id: String,
    pub resources: DeploymentResource,
    pub metadata: Option<DeploymentMetadata>,
}

impl ResultFormatting for DeploymentGetResponse {
    fn compact(&self) -> String {
        format!("{}/{}: healthy: {}", self.name, self.id, self.healthy)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentObservability {
    /// Whether the deployment observability is healthy or not (one or more of the subsections will have healthy: false)
    pub healthy: bool,
    pub metrics: Option<DeploymentMetrics>,
    pub logging: Option<DeploymentLogging>,
    /// General observability health issues for the deployment
    pub issues: Option<Vec<ObservabilityIssue>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentMetrics {
    /// Whether the deployment metrics are healthy or not
    pub healthy: bool,
    /// Metrics health issues for the deployment
    pub issues: Option<Vec<ObservabilityIssue>>,
    /// The URLs to view this deployment's metrics in Kibana
    pub urls: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentLogging {
    /// Whether the deployment logging is healthy or not
    pub healthy: bool,
    /// Logging health issues for the deployment
    pub issues: Option<Vec<ObservabilityIssue>>,
    /// The URLs to view this deployment's logs in Kibana
    pub urls: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityIssue {
    /// A user-friendly description of the observability health issue
    pub description: String,
    /// Severity of the health issue
    pub severity: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentSettings {
    /// If autoscaling is enabled for this deployment.
    pub autoscaling_enabled: Option<bool>,
    pub observability: Option<DeploymentObservabilitySettings>,
    pub traffic_filter_settings: Option<TrafficFilterSettings>,
}




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentLoggingSettings {
    pub destination: ObservabilityAbsoluteDeployment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentMetricsSettings {
    pub destination: ObservabilityAbsoluteDeployment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityAbsoluteDeployment {
    /// The deployment to send logs and/or metrics to. Contains either the deployment's ID or 'self'.
    pub deployment_id: String,
    /// RefId of the Elasticsearch cluster to send logs and/or metrics to. If not specified, refId is resolved automatically as long as the destination deployment contains a single Elasticsearch resource.
    pub ref_id: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentResource {
    /// The Elasticsearch cluster that this resource depends on.
    pub elasticsearch_cluster_ref_id: Option<String>,
    /// The kind of resource
    pub kind: Option<String>,
    /// An encoded string that provides other Elastic services with the necessary information to connect to this Elasticsearch and Kibana
    pub cloud_id: Option<String>,
    /// Secret token for using a created resource. Only provided on initial create and absent otherwise.
    pub secret_token: Option<String>,
    /// List of warnings generated from validating resource updates
    pub warnings: Option<Vec<ReplyWarning>>,
    /// Identifier of the region in which this resource runs.
    pub region: Option<String>,
    /// A locally-unique friendly alias for this Elasticsearch cluster
    pub ref_id: Option<String>,
    pub credentials: Option<ClusterCredentials>,
    /// A system-unique id for the created resource
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCredentials {
    /// The username of the newly created cluster
    pub username: String,
    /// The password of the newly created cluster
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplyWarning {
    /// A human readable message describing the warning that occurred
    pub message: Option<String>,
    /// A structured code representing the error type that occurred
    pub code: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentShutdownResponse {
    /// The id of the deployment
    pub id: String,
    pub orphaned: Option<Orphaned>,
    /// The name of the deployment
    pub name: String,
}

impl ResultFormatting for DeploymentShutdownResponse{
    fn compact(&self) -> String {
        format!("{}: {}", self.id, self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Orphaned {
    /// List of orphaned Enterprise Search resource ids
    pub enterprise_search: Vec<String>,
    /// List of orphaned Kibana resource ids
    pub kibana: Vec<String>,
    /// List of orphaned Elasticsearch resources
    pub elasticsearch: Vec<OrphanedElasticsearch>,
    /// List of orphaned APM resource ids
    pub apm: Vec<String>,
    /// List of orphaned AppSearch resource ids
    pub appsearch: Vec<String>,
    /// List of orphaned Integrations Server resource ids
    pub integrations_server: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrphanedElasticsearch {
    /// List of orphaned dependent resources
    pub dependents: Vec<ElasticsearchDependant>,
    /// The id of the orphaned resource
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchDependant {
    /// The kind of resource
    pub kind: String,
    /// The id of the orphaned resource
    pub id: String,
}



// ***********************************************************************
// common structs


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficFilterSettings {
    /// IDs of the traffic filter rulesets
    pub rulesets: Vec<String>,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentObservabilitySettings {
    pub metrics: Option<DeploymentMetricsSettings>,
    pub logging: Option<DeploymentLoggingSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentMetadata {
    /// Arbitrary user-defined metadata associated with this deployment
    pub tags: Option<Vec<MetadataItem>>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataItem {
    /// The metadata value
    pub value: String,
    /// The metadata field name
    pub key: String,
}