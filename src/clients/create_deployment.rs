
// ***********************************************************************
// structs for deployment creation 
// stateful deployment creation is hilariously complex, with tons of fields.
// the eventual goal is to have some kind of derive macro that implements a getter/setter
// that can recursively reach into the structs. Until then, the CLI just has to implement whatever options it thinks are important.

use serde::{Serialize, Deserialize};

use super::deployments::{DeploymentMetadata, DeploymentObservabilitySettings, TrafficFilterSettings, DeploymentResource};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCreateResponse {
    /// The name of the deployment
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not the deployment was freshly created
    #[serde(rename = "created")]
    pub created: bool,
    /// A user-defined deployment alias for user-friendly resource URLs
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "diagnostics", skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DeploymentDiagnostics>,
    /// The id of the deployment
    #[serde(rename = "id")]
    pub id: String,
    /// List of created resources.
    #[serde(rename = "resources")]
    pub resources: Vec<DeploymentResource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentDiagnostics {
    #[serde(rename = "creates", skip_serializing_if = "Option::is_none")]
    pub creates: Option<Creates>,
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Updates>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Updates {
    /// Diagnostics for Enterprise Search resources
    #[serde(rename = "enterprise_search", skip_serializing_if = "Option::is_none")]
    pub enterprise_search: Option<Vec<EnterpriseSearch>>,
    /// Diagnostics for Kibanas
    #[serde(rename = "kibana", skip_serializing_if = "Option::is_none")]
    pub kibana: Option<Vec<Kibana>>,
    /// Diagnostics for Elasticsearch clusters
    #[serde(rename = "elasticsearch", skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<Vec<Elasticsearch>>,
    /// Diagnostics for APMs
    #[serde(rename = "apm", skip_serializing_if = "Option::is_none")]
    pub apm: Option<Vec<Apm>>,
    /// Diagnostics for AppSearches
    #[serde(rename = "appsearch", skip_serializing_if = "Option::is_none")]
    pub appsearch: Option<Vec<AppSearch>>,
    /// Diagnostics for Integrations Server
    #[serde(rename = "integrations_server", skip_serializing_if = "Option::is_none")]
    pub integrations_server: Option<Vec<IntegrationsServer>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Creates {
    /// Diagnostics for Enterprise Search resources
    #[serde(rename = "enterprise_search", skip_serializing_if = "Option::is_none")]
    pub enterprise_search: Option<Vec<EnterpriseSearch>>,
    /// Diagnostics for Kibanas
    #[serde(rename = "kibana", skip_serializing_if = "Option::is_none")]
    pub kibana: Option<Vec<Kibana>>,
    /// Diagnostics for Elasticsearch clusters
    #[serde(rename = "elasticsearch", skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<Vec<Elasticsearch>>,
    /// Diagnostics for APMs
    #[serde(rename = "apm", skip_serializing_if = "Option::is_none")]
    pub apm: Option<Vec<Apm>>,
    /// Diagnostics for AppSearches
    #[serde(rename = "appsearch", skip_serializing_if = "Option::is_none")]
    pub appsearch: Option<Vec<AppSearch>>,
    /// Diagnostics for Integrations Server
    #[serde(rename = "integrations_server", skip_serializing_if = "Option::is_none")]
    pub integrations_server: Option<Vec<IntegrationsServer>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServer {
    /// The user-specified id of the Elasticsearch Cluster that this will link to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearch {
    /// The user-specified id of the Elasticsearch Cluster that this will link to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Apm {
    /// The user-specified id of the Elasticsearch Cluster that this will link to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Elasticsearch {
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name for the cluster (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Kibana {
    /// The user-specified id of the Elasticsearch Cluster that this will link to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearch {
    /// The user-specified id of the Elasticsearch Cluster that this will link to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The backend plan as JSON
    #[serde(rename = "backend_plan")]
    pub backend_plan: serde_json::Value,
    /// The human readable name (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A locally-unique user-specified id
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCreateRequest {
    /// A name for the deployment; otherwise this will be the generated deployment id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DeploymentCreateSettings>,
    /// Identifier of the region to be used as the default for all the resources of the deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// A user-defined alias to use in place of Cluster IDs for user-friendly URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// The version for all the resources of the deployment (must be one of the supported versions). Defaults to the latest version if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<DeploymentCreateResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DeploymentMetadata>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCreateSettings {
    /// Enable autoscaling for this deployment.
    pub autoscaling_enabled: Option<bool>,
    pub observability: Option<DeploymentObservabilitySettings>,
    pub traffic_filter_settings: Option<TrafficFilterSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCreateResources {
    /// A list of payloads for Enterprise Search creation.
    #[serde(rename = "enterprise_search", skip_serializing_if = "Option::is_none")]
    pub enterprise_search: Option<Vec<EnterpriseSearchPayload>>,
    /// A list of payloads for Kibana creation.
    #[serde(rename = "kibana", skip_serializing_if = "Option::is_none")]
    pub kibana: Option<Vec<KibanaPayload>>,
    /// A list of payloads for Elasticsearch cluster creation.
    #[serde(rename = "elasticsearch", skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<Vec<ElasticsearchPayload>>,
    /// A list of payloads for APM creation. WARNING: For stack versions 8.0.0 and higher the integrations_server payload should be used instead, as this field becomes deprecated.
    // #[serde(rename = "apm", skip_serializing_if = "Option::is_none")]
    // pub apm: Option<Vec<crate::models::ApmPayload>>,
    /// A list of payloads for AppSearch updates. AppSearch has been replaced by Enterprise Search in the Elastic Stack 7.7 and higher.
    #[serde(rename = "appsearch", skip_serializing_if = "Option::is_none")]
    pub appsearch: Option<Vec<AppSearchPayload>>,
    /// A list of payloads for Integrations Server creation.
    #[serde(rename = "integrations_server", skip_serializing_if = "Option::is_none")]
    pub integrations_server: Option<Vec<IntegrationsServerPayload>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerPayload {
    /// Alias to the Elasticsearch Cluster to attach the Integrations Server to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The human readable name for the Integrations Server cluster (default: takes the name of its Elasticsearch cluster)
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<IntegrationsServerSettings>,
    /// The region where this resource exists
    #[serde(rename = "region")]
    pub region: String,
    /// A locally-unique user-specified id for the Integrations Server
    #[serde(rename = "ref_id")]
    pub ref_id: String,
    #[serde(rename = "plan")]
    pub plan: IntegrationsServerPlan,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerPlan {
    #[serde(rename = "cluster_topology", skip_serializing_if = "Option::is_none")]
    pub cluster_topology: Option<Vec<IntegrationsServerTopologyElement>>,
    #[serde(rename = "transient", skip_serializing_if = "Option::is_none")]
    pub transient: Option<TransientIntegrationsServerPlanConfiguration>,
    #[serde(rename = "integrations_server")]
    pub integrations_server: IntegrationsServerConfiguration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransientIntegrationsServerPlanConfiguration {
    #[serde(rename = "plan_configuration", skip_serializing_if = "Option::is_none")]
    pub plan_configuration: Option<IntegrationsServerPlanControlConfiguration>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<PlanStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerPlanControlConfiguration {
    /// Set to 'forced' to force a reboot as part of the upgrade plan
    #[serde(rename = "cluster_reboot", skip_serializing_if = "Option::is_none")]
    pub cluster_reboot: Option<ClusterReboot>,
    /// If true (default false), does not clear the maintenance flag (which prevents its API from being accessed except by the constructor) on new instances added until after a snapshot has been restored, otherwise, the maintenance flag is cleared once the new instances successfully join the new cluster
    #[serde(rename = "extended_maintenance", skip_serializing_if = "Option::is_none")]
    pub extended_maintenance: Option<bool>,
    /// This timeout determines how long to give a cluster after it responds to API calls before performing actual operations on it. It defaults to 5s
    #[serde(rename = "calm_wait_time", skip_serializing_if = "Option::is_none")]
    pub calm_wait_time: Option<i64>,
    /// The total timeout in seconds after which the plan is cancelled even if it is not complete. Defaults to 4x the max memory capacity per node (in MB)
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerTopologyElement {
    /// The version of the Instance Configuration Id. Unset for unversioned Instance Configurations on read. If unset in creates, means most recent version. If unset in updates, means keep the same version.
    #[serde(rename = "instance_configuration_version", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_version: Option<i32>,
    /// Controls the allocation of this topology element as well as allowed sizes and node_types. It needs to match the id of an existing instance configuration.
    #[serde(rename = "instance_configuration_id", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_id: Option<String>,
    /// number of zones in which nodes will be placed
    #[serde(rename = "zone_count", skip_serializing_if = "Option::is_none")]
    pub zone_count: Option<i32>,
    #[serde(rename = "integrations_server", skip_serializing_if = "Option::is_none")]
    pub integrations_server: Option<IntegrationsServerConfiguration>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<TopologySize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerConfiguration {
    /// An arbitrary JSON object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_yaml' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Integrations Server settings)
    #[serde(rename = "user_settings_override_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_json: Option<serde_json::Value>,
    /// An arbitrary YAML object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_json' is allowed), provided the parameters are on the allowlist and not on the denylist. (These field together with 'user_settings_override*' and 'system_settings' defines the total set of Integrations Server settings)
    #[serde(rename = "user_settings_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_yaml: Option<String>,
    /// The version of the Integrations Server cluster (must be one of the ECE supported versions, and won't work unless it matches the Integrations Server version. Leave blank to auto-detect version.)
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The mode the Integrations Server is operating in.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// An arbitrary JSON object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_yaml' is allowed), provided the parameters are on the allowlist and not on the denylist. (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Integrations Server settings)
    #[serde(rename = "user_settings_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_json: Option<serde_json::Value>,
    #[serde(rename = "system_settings", skip_serializing_if = "Option::is_none")]
    pub system_settings: Option<IntegrationsServerSystemSettings>,
    /// An arbitrary YAML object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_json' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Integrations Server settings)
    #[serde(rename = "user_settings_override_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_yaml: Option<String>,
    /// A docker URI that allows overriding of the default docker image specified for this version
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerSystemSettings {
    /// DEPRECATED: Scheduled for removal in a future version of the API.  Optionally override the URL to which to send data (for advanced users only, if unspecified the system selects an internal URL)
    #[serde(rename = "elasticsearch_url", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_url: Option<String>,
    /// Optionally override the secret token within Integrations Server - defaults to the previously existing secretToken
    #[serde(rename = "secret_token", skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
    /// Optionally override the account within Integrations Server - defaults to a system account that always exists (if specified, the username must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_password", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_password: Option<String>,
    /// Optionally enable debug mode for Integrations Server - defaults false
    #[serde(rename = "debug_enabled", skip_serializing_if = "Option::is_none")]
    pub debug_enabled: Option<bool>,
    /// DEPRECATED: Scheduled for removal in a future version of the API.  Optionally override the URL to which to send data (for advanced users only, if unspecified the system selects an internal URL)
    #[serde(rename = "kibana_url", skip_serializing_if = "Option::is_none")]
    pub kibana_url: Option<String>,
    /// Optionally override the account within Integrations Server - defaults to a system account that always exists (if specified, the password must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_username", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_username: Option<String>,
}

/// The mode the Integrations Server is operating in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "standalone")]
    Standalone,
    #[serde(rename = "managed")]
    Managed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationsServerSettings {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterMetadataSettings>,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchPayload {
    /// Alias to the Elasticsearch Cluster to attach AppSearch to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The human readable name for the AppSearch cluster (default: takes the name of its Elasticsearch cluster)
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<AppSearchSettings>,
    /// The region where this resource exists
    #[serde(rename = "region")]
    pub region: String,
    /// A locally-unique user-specified id for AppSearch
    #[serde(rename = "ref_id")]
    pub ref_id: String,
    #[serde(rename = "plan")]
    pub plan: AppSearchPlan,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchSettings {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterMetadataSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchPlan {
    #[serde(rename = "appsearch")]
    pub appsearch: AppSearchConfiguration,
    #[serde(rename = "cluster_topology", skip_serializing_if = "Option::is_none")]
    pub cluster_topology: Option<Vec<AppSearchTopologyElement>>,
    #[serde(rename = "transient", skip_serializing_if = "Option::is_none")]
    pub transient: Option<TransientAppSearchPlanConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchTopologyElement {
    /// number of zones in which nodes will be placed
    #[serde(rename = "zone_count", skip_serializing_if = "Option::is_none")]
    pub zone_count: Option<i32>,
    /// The version of the Instance Configuration Id. Unset for unversioned Instance Configurations on read. If unset in creates, means most recent version. If unset in updates, means keep the same version.
    #[serde(rename = "instance_configuration_version", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_version: Option<i32>,
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<AppSearchNodeTypes>,
    #[serde(rename = "appsearch", skip_serializing_if = "Option::is_none")]
    pub appsearch: Option<AppSearchConfiguration>,
    /// Controls the allocation of this topology element as well as allowed sizes and node_types. It needs to match the id of an existing instance configuration.
    #[serde(rename = "instance_configuration_id", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_id: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<TopologySize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchNodeTypes {
    /// Defines whether this instance should run as Application/API server
    #[serde(rename = "appserver")]
    pub appserver: bool,
    /// Defines whether this instance should run as background worker
    #[serde(rename = "worker")]
    pub worker: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransientAppSearchPlanConfiguration {
    #[serde(rename = "plan_configuration", skip_serializing_if = "Option::is_none")]
    pub plan_configuration: Option<AppSearchPlanControlConfiguration>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<PlanStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchPlanControlConfiguration {
    /// Set to 'forced' to force a reboot as part of the upgrade plan
    #[serde(rename = "cluster_reboot", skip_serializing_if = "Option::is_none")]
    pub cluster_reboot: Option<ClusterReboot>,
    #[serde(rename = "move_allocators", skip_serializing_if = "Option::is_none")]
    pub move_allocators: Option<Vec<AllocatorMoveRequest>>,
    /// If true (default: false) does not allow re-using any existing instances currently in the cluster, ie even unchanged instances will be re-created
    #[serde(rename = "reallocate_instances", skip_serializing_if = "Option::is_none")]
    pub reallocate_instances: Option<bool>,
    /// List of allocators on which instances are placed if possible (if not possible/not specified then any available allocator with space is used)
    #[serde(rename = "preferred_allocators", skip_serializing_if = "Option::is_none")]
    pub preferred_allocators: Option<Vec<String>>,
    /// This timeout determines how long to give a cluster after it responds to API calls before performing actual operations on it. It defaults to 5s
    #[serde(rename = "calm_wait_time", skip_serializing_if = "Option::is_none")]
    pub calm_wait_time: Option<i64>,
    /// The total timeout in seconds after which the plan is cancelled even if it is not complete. Defaults to 4x the max memory capacity per node (in MB)
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// If true (default false), does not clear the maintenance flag (which prevents its API from being accessed except by the constructor) on new instances added until after a snapshot has been restored, otherwise, the maintenance flag is cleared once the new instances successfully join the new cluster
    #[serde(rename = "extended_maintenance", skip_serializing_if = "Option::is_none")]
    pub extended_maintenance: Option<bool>,
    #[serde(rename = "move_instances", skip_serializing_if = "Option::is_none")]
    pub move_instances: Option<Vec<InstanceMoveRequest>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchConfiguration {
    /// An arbitrary JSON object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_yaml' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of AppSearch settings)
    #[serde(rename = "user_settings_override_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_json: Option<serde_json::Value>,
    /// An arbitrary YAML object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_json' is allowed), provided the parameters are on the allowlist and not on the denylist. (These field together with 'user_settings_override*' and 'system_settings' defines the total set of AppSearch settings)
    #[serde(rename = "user_settings_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_yaml: Option<String>,
    /// The version of the AppSearch cluster (must be one of the ECE supported versions, and won't work unless it matches the Elasticsearch version. Leave blank to auto-detect version.)
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// An arbitrary JSON object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_yaml' is allowed), provided the parameters are on the allowlist and not on the denylist. (This field together with 'user_settings_override*' and 'system_settings' defines the total set of AppSearch settings)
    #[serde(rename = "user_settings_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_json: Option<serde_json::Value>,
    #[serde(rename = "system_settings", skip_serializing_if = "Option::is_none")]
    pub system_settings: Option<AppSearchSystemSettings>,
    /// An arbitrary YAML object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_json' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of AppSearch settings)
    #[serde(rename = "user_settings_override_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_yaml: Option<String>,
    /// A docker URI that allows overriding of the default docker image specified for this version
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSearchSystemSettings {
    /// Optionally override the account within App Search - defaults to a system account that always exists (if specified, the username must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_password", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_password: Option<String>,
    /// Optionally override the account within App Search - defaults to a system account that always exists (if specified, the password must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_username", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_username: Option<String>,
    /// Optionally override the secret session key within App Search - defaults to the previously existing secretSession. Note that this field is never returned from the API, it is write only.
    #[serde(rename = "secret_session_key", skip_serializing_if = "Option::is_none")]
    pub secret_session_key: Option<String>,
    /// Optionally override the URL to which to send data (for advanced users only, if unspecified the system selects an internal URL)
    #[serde(rename = "elasticsearch_url", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchPayload {
    /// The region where this resource exists
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<ElasticsearchClusterSettings>,
    /// The human readable name for the cluster (defaults to the generated cluster id if not specified)
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "plan")]
    pub plan: ElasticsearchClusterPlan,
    /// A locally-unique user-specified id for this Elasticsearch cluster
    #[serde(rename = "ref_id")]
    pub ref_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchClusterPlan {
    /// Enable autoscaling for this Elasticsearch cluster.
    #[serde(rename = "autoscaling_enabled", skip_serializing_if = "Option::is_none")]
    pub autoscaling_enabled: Option<bool>,
    #[serde(rename = "cluster_topology")]
    pub cluster_topology: Vec<ElasticsearchClusterTopologyElement>,
    #[serde(rename = "transient", skip_serializing_if = "Option::is_none")]
    pub transient: Option<TransientElasticsearchPlanConfiguration>,
    #[serde(rename = "elasticsearch")]
    pub elasticsearch: ElasticsearchConfiguration,
    #[serde(rename = "deployment_template", skip_serializing_if = "Option::is_none")]
    pub deployment_template: Option<DeploymentTemplateReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransientElasticsearchPlanConfiguration {
    #[serde(rename = "plan_configuration", skip_serializing_if = "Option::is_none")]
    pub plan_configuration: Option<ElasticsearchPlanControlConfiguration>,
    /// If specified, contains transient settings to be applied to an Elasticsearch cluster during changes,default values shown below applied. These can be overridden by specifying them in the map (or null to unset). Additional settings can also be set. Settings will be cleared after the plan has finished. If not specified, no settings will be applied. NOTE: These settings are only explicitly cleared for 5.x+ clusters, they must be hand-reset to their defaults in 2.x- (or a cluster reboot will clear them). - indices.store.throttle.max_bytes_per_sec: 120Mb - indices.recovery.max_bytes_per_sec: 120Mb - cluster.routing.allocation.cluster_concurrent_rebalance: 5 - cluster.routing.allocation.node_initial_primaries_recoveries: 5 - cluster.routing.allocation.node_concurrent_incoming_recoveries: 5 For version 8.1 and later no defaults are provided through this mechanism, but instead hardware dependent settings are provided to each instance.
    #[serde(rename = "cluster_settings_json", skip_serializing_if = "Option::is_none")]
    pub cluster_settings_json: Option<serde_json::Value>,
    #[serde(rename = "remote_clusters", skip_serializing_if = "Option::is_none")]
    pub remote_clusters: Option<RemoteResources>,
    #[serde(rename = "restore_snapshot", skip_serializing_if = "Option::is_none")]
    pub restore_snapshot: Option<RestoreSnapshotConfiguration>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<PlanStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreSnapshotConfiguration {
    #[serde(rename = "repository_config", skip_serializing_if = "Option::is_none")]
    pub repository_config: Option<RestoreSnapshotRepoConfiguration>,
    /// If specified, contains the name of the snapshot repository - else will default to the Elastic Cloud system repo ('found-snapshots')
    #[serde(rename = "repository_name", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "restore_payload", skip_serializing_if = "Option::is_none")]
    pub restore_payload: Option<RestoreSnapshotApiConfiguration>,
    /// The restore strategy to use. Defaults to a full restore. Partial restore will attempt to restore unavailable indices only
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    /// If specified, contains the name of the source cluster id. Do not send this if you are sending repository_config
    #[serde(rename = "source_cluster_id", skip_serializing_if = "Option::is_none")]
    pub source_cluster_id: Option<String>,
    /// The name of the snapshot to restore. Use '\\_\\_latest_success\\_\\_' to get the most recent snapshot from the specified repository
    #[serde(rename = "snapshot_name")]
    pub snapshot_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreSnapshotApiConfiguration {
    /// The list of indices to restore (supports +ve and -ve selection and wildcarding - see the default Elasticsearch index format documentation)
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<String>>,
    /// This JSON object (merged with the 'indices' field (if present) is passed untouched into the restore command - see the Elasticsearch '_snapshot' documentation for more details on supported formats
    #[serde(rename = "raw_settings", skip_serializing_if = "Option::is_none")]
    pub raw_settings: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreSnapshotRepoConfiguration {
    /// The remote snapshot settings raw JSON - see the Elasticsearch '_snapshot' documentation for more details on supported formats
    #[serde(rename = "raw_settings", skip_serializing_if = "Option::is_none")]
    pub raw_settings: Option<serde_json::Value>,
}

/// The restore strategy to use. Defaults to a full restore. Partial restore will attempt to restore unavailable indices only
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "recovery")]
    Recovery,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteResources {
    /// The remote resources
    #[serde(rename = "resources")]
    pub resources: Vec<RemoteResourceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteResourceRef {
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<RemoteResourceInfo>,
    /// The locally-unique user-specified id of an Elasticsearch Resource
    #[serde(rename = "elasticsearch_ref_id")]
    pub elasticsearch_ref_id: String,
    /// The id of the deployment
    #[serde(rename = "deployment_id")]
    pub deployment_id: String,
    /// If true, skip this cluster during search if it is disconnected. Default: false
    #[serde(rename = "skip_unavailable", skip_serializing_if = "Option::is_none")]
    pub skip_unavailable: Option<bool>,
    /// The alias for this remote cluster. Aliases must only contain letters, digits, dashes and underscores
    #[serde(rename = "alias")]
    pub alias: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteResourceInfo {
    /// Whether or not the remote cluster is healthy
    #[serde(rename = "healthy")]
    pub healthy: bool,
    /// Whether or not the remote cluster version is compatible with this cluster version.
    #[serde(rename = "compatible")]
    pub compatible: bool,
    /// Whether or not there is at least one connection to the remote cluster.
    #[serde(rename = "connected")]
    pub connected: bool,
    /// Whether or not the remote cluster is trusted by this cluster.
    #[serde(rename = "trusted")]
    pub trusted: bool,
    /// Whether or not the remote cluster trusts this cluster back.
    #[serde(rename = "trusted_back")]
    pub trusted_back: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchPlanControlConfiguration {
    /// Set to 'forced' to force a reboot as part of the upgrade plan. NOTES: (ie taking an existing plan and leaving it alone except for setting 'transient.plan_configuration.cluster_reboot': 'forced' will reboot the cluster)
    #[serde(rename = "cluster_reboot", skip_serializing_if = "Option::is_none")]
    pub cluster_reboot: Option<ClusterReboot>,
    /// This timeout determines how long to give a cluster after it responds to API calls before performing actual operations on it. It defaults to 5s
    #[serde(rename = "calm_wait_time", skip_serializing_if = "Option::is_none")]
    pub calm_wait_time: Option<i64>,
    /// If true (default: false), does not take (or require) a successful snapshot to be taken before performing any potentially destructive changes to this cluster
    #[serde(rename = "skip_snapshot", skip_serializing_if = "Option::is_none")]
    pub skip_snapshot: Option<bool>,
    /// When you take a snapshot and 'skip_snapshots' is false, specifies the maximum age in seconds of the most recent snapshot before a new snapshot is created. Default is 300
    #[serde(rename = "max_snapshot_age", skip_serializing_if = "Option::is_none")]
    pub max_snapshot_age: Option<i64>,
    /// The total timeout in seconds after which the plan is cancelled even if it is not complete. Defaults to 4x the max memory capacity per node (in MB). NOTES: A 3 zone cluster with 2 nodes of 2048 each would have a timeout of 4*2048=8192 seconds. Timeout does not include time required to run rollback actions.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// If true (default false), does not clear the maintenance flag (which prevents its API from being accessed except by the constructor) on new instances added until after a snapshot has been restored, otherwise, the maintenance flag is cleared once the new instances successfully join the new cluster
    #[serde(rename = "extended_maintenance", skip_serializing_if = "Option::is_none")]
    pub extended_maintenance: Option<bool>,
    /// If taking a snapshot (ie unless 'skip_snapshots': true) then will retry on failure at most this number of times (default: 5)
    #[serde(rename = "max_snapshot_attempts", skip_serializing_if = "Option::is_none")]
    pub max_snapshot_attempts: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchClusterTopologyElement {
    /// An arbitrary JSON object overriding the default autoscaling policy. Don't set unless you really know what you are doing.
    #[serde(rename = "autoscaling_policy_override_json", skip_serializing_if = "Option::is_none")]
    pub autoscaling_policy_override_json: Option<serde_json::Value>,
    /// The default number of zones in which data nodes will be placed
    #[serde(rename = "zone_count", skip_serializing_if = "Option::is_none")]
    pub zone_count: Option<i32>,
    #[serde(rename = "topology_element_control", skip_serializing_if = "Option::is_none")]
    pub topology_element_control: Option<TopologyElementControl>,
    /// The version of the Instance Configuration Id. Unset for unversioned Instance Configurations on read. If unset in cluster or tier creates or if changing IC id, means most recent version. If unset in other updates, means keep the same version.
    #[serde(rename = "instance_configuration_version", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_version: Option<i32>,
    #[serde(rename = "autoscaling_min", skip_serializing_if = "Option::is_none")]
    pub autoscaling_min: Option<TopologySize>,
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<ElasticsearchNodeType>,
    #[serde(rename = "elasticsearch", skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<ElasticsearchConfiguration>,
    #[serde(rename = "autoscaling_max", skip_serializing_if = "Option::is_none")]
    pub autoscaling_max: Option<TopologySize>,
    /// Controls the allocation of this topology element as well as allowed sizes and node_types. It needs to match the id of an existing instance configuration.
    #[serde(rename = "instance_configuration_id", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_id: Option<String>,
    /// The list of node roles for this topology element (ES version >= 7.10). Allowable values are: master, ingest, ml, data_hot, data_content, data_warm, data_cold, data_frozen, remote_cluster_client, transform
    #[serde(rename = "node_roles", skip_serializing_if = "Option::is_none")]
    pub node_roles: Option<Vec<NodeRoles>>,
    /// Unique identifier of this topology element
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<TopologySize>,
}

/// The list of node roles for this topology element (ES version >= 7.10). Allowable values are: master, ingest, ml, data_hot, data_content, data_warm, data_cold, data_frozen, remote_cluster_client, transform
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeRoles {
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "ingest")]
    Ingest,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "data_hot")]
    DataHot,
    #[serde(rename = "data_content")]
    DataContent,
    #[serde(rename = "data_warm")]
    DataWarm,
    #[serde(rename = "data_cold")]
    DataCold,
    #[serde(rename = "data_frozen")]
    DataFrozen,
    #[serde(rename = "remote_cluster_client")]
    RemoteClusterClient,
    #[serde(rename = "transform")]
    Transform,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchNodeType {
    /// Defines whether this node can hold data (default: false)
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// Defines whether this node can be elected master (default: false)
    #[serde(rename = "master", skip_serializing_if = "Option::is_none")]
    pub master: Option<bool>,
    /// Defines whether this node can run an ingest pipeline (default: false)
    #[serde(rename = "ingest", skip_serializing_if = "Option::is_none")]
    pub ingest: Option<bool>,
    /// Defines whether this node can run ml jobs, valid only for versions 5.4.0 or greater (default: false)
    #[serde(rename = "ml", skip_serializing_if = "Option::is_none")]
    pub ml: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyElementControl {
    #[serde(rename = "min")]
    pub min: TopologySize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologySize {
    /// Type of resource
    #[serde(rename = "resource")]
    pub resource: Resource,
    /// Amount of resource
    #[serde(rename = "value")]
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchConfiguration {
    /// An arbitrary JSON object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_yaml' is allowed), ie in addition to the documented 'system_settings'. NOTES: (This field together with 'system_settings' and 'user_settings*' defines the total set of Elasticsearch settings)
    #[serde(rename = "user_settings_override_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_json: Option<serde_json::Value>,
    /// A list of plugin names from the Elastic-supported subset that are bundled with the version images. NOTES: (Users should consult the Elastic stack objects to see what plugins are available, this is currently only available from the UI)
    #[serde(rename = "enabled_built_in_plugins", skip_serializing_if = "Option::is_none")]
    pub enabled_built_in_plugins: Option<Vec<String>>,
    /// A list of admin-uploaded plugin objects that are available for this user.
    #[serde(rename = "user_plugins", skip_serializing_if = "Option::is_none")]
    pub user_plugins: Option<Vec<ElasticsearchUserPlugin>>,
    /// An arbitrary YAML object allowing cluster owners to set their parameters (only one of this and 'user_settings_json' is allowed), provided the parameters arey are on the allowlist and not on the denylist. NOTES: (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Elasticsearch settings)
    #[serde(rename = "user_settings_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_yaml: Option<String>,
    /// A list of admin-uploaded bundle objects (eg scripts, synonym files) that are available for this user.
    #[serde(rename = "user_bundles", skip_serializing_if = "Option::is_none")]
    pub user_bundles: Option<Vec<ElasticsearchUserBundle>>,
    /// The version of the Elasticsearch cluster (must be one of the ECE supported versions). Currently cannot be different across the topology (and is generally specified in the globals). Defaults to the latest version if not specified.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// An arbitrary JSON object allowing cluster owners to set their parameters (only one of this and 'user_settings_yaml' is allowed), provided the parameters arey are on the allowlist and not on the denylist. NOTES: (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Elasticsearch settings)
    #[serde(rename = "user_settings_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_json: Option<serde_json::Value>,
    #[serde(rename = "curation", skip_serializing_if = "Option::is_none")]
    pub curation: Option<ElasticsearchCuration>,
    #[serde(rename = "system_settings", skip_serializing_if = "Option::is_none")]
    pub system_settings: Option<ElasticsearchSystemSettings>,
    /// An arbitrary YAML object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_json' is allowed), ie in addition to the documented 'system_settings'. NOTES: (This field together with 'system_settings' and 'user_settings*' defines the total set of Elasticsearch settings)
    #[serde(rename = "user_settings_override_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_yaml: Option<String>,
    /// A docker URI that allows overriding of the default docker image specified for this version
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    /// Defines the Elasticsearch node attributes for the instances in the topology
    #[serde(rename = "node_attributes", skip_serializing_if = "Option::is_none")]
    pub node_attributes: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchSystemSettings {
    /// Limits remote Elasticsearch clusters that can be used as the source for '_reindex' API commands
    #[serde(rename = "reindex_whitelist", skip_serializing_if = "Option::is_none")]
    pub reindex_whitelist: Option<Vec<String>>,
    /// (2.x only - to get the same result in 5.x template mappings must be used) Sets the default number of shards per index, defaulting to 1 if not specified. (Corresponds to the parameter 'index.number_of_shards' in 2.x, not supported in 5.x)
    #[serde(rename = "default_shards_per_index", skip_serializing_if = "Option::is_none")]
    pub default_shards_per_index: Option<i32>,
    /// The duration for which monitoring history is stored (format '(NUMBER)d' eg '3d' for 3 days). NOTES: ('Corresponds to the parameter xpack.monitoring.history.duration' in 5.x, defaults to '7d')
    #[serde(rename = "monitoring_history_duration", skip_serializing_if = "Option::is_none")]
    pub monitoring_history_duration: Option<String>,
    /// The default interval at which monitoring information from the cluster if collected, if monitoring is enabled. NOTES: (Corresponds to the parameter 'marvel.agent.interval' in 2.x and 'xpack.monitoring.collection.interval' in 5.x)
    #[serde(rename = "monitoring_collection_interval", skip_serializing_if = "Option::is_none")]
    pub monitoring_collection_interval: Option<i32>,
    /// If true (default is false) then the index deletion API will not support wildcards or '_all'. NOTES: (Corresponds to the parameter 'action.destructive_requires_name')
    #[serde(rename = "destructive_requires_name", skip_serializing_if = "Option::is_none")]
    pub destructive_requires_name: Option<bool>,
    /// If true (the default), then any write operation on an index that does not currently exist will create it. NOTES: (Corresponds to the parameter 'action.auto_create_index')
    #[serde(rename = "auto_create_index", skip_serializing_if = "Option::is_none")]
    pub auto_create_index: Option<bool>,
    /// The trigger engine for Watcher, defaults to 'scheduler' - see the xpack documentation for more information. NOTES: (Corresponds to the parameter '(xpack.)watcher.trigger.schedule.engine', depending on version. Ignored from 6.x onwards.)
    #[serde(rename = "watcher_trigger_engine", skip_serializing_if = "Option::is_none")]
    pub watcher_trigger_engine: Option<String>,
    #[serde(rename = "scripting", skip_serializing_if = "Option::is_none")]
    pub scripting: Option<ElasticsearchScriptingUserSettings>,
    /// Defaults to false on versions <= 7.2.0, true otherwise. If false, then the API commands to close indices are disabled. This is important because Elasticsearch does not snapshot or migrate close indices on versions under 7.2.0, therefore standard Elastic Cloud configuration operations will cause irretrievable loss of indices' data. NOTES: (Corresponds to the parameter 'cluster.indices.close.enable')
    #[serde(rename = "enable_close_index", skip_serializing_if = "Option::is_none")]
    pub enable_close_index: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchScriptingUserSettings {
    /// (5.x+ only) If enabled (the default) then the expressions scripting engine is allowed as a sandboxed language. Sandboxed languages are the only ones allowed if 'sandbox_mode' is set to true. NOTES: (Corresponds to the parameters 'script.engine.expression.[file|stored|inline]')
    #[serde(rename = "expressions_enabled", skip_serializing_if = "Option::is_none")]
    pub expressions_enabled: Option<bool>,
    #[serde(rename = "stored", skip_serializing_if = "Option::is_none")]
    pub stored: Option<ElasticsearchScriptTypeSettings>,
    /// (5.x+ only) If enabled (the default) then the painless scripting engine is allowed as a sandboxed language. Sandboxed languages are the only ones allowed if 'sandbox_mode' is set to true. NOTES: (Corresponds to the parameters 'script.engine.painless.[file|stored|inline]')
    #[serde(rename = "painless_enabled", skip_serializing_if = "Option::is_none")]
    pub painless_enabled: Option<bool>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<ElasticsearchScriptTypeSettings>,
    #[serde(rename = "inline", skip_serializing_if = "Option::is_none")]
    pub inline: Option<ElasticsearchScriptTypeSettings>,
    /// (5.x+ only) If enabled (the default) then the mustache scripting engine is allowed as a sandboxed language. Sandboxed languages are the only ones allowed if 'sandbox_mode' is set to true. NOTES: (Corresponds to the parameters 'script.engine.mustache.[file|stored|inline]')
    #[serde(rename = "mustache_enabled", skip_serializing_if = "Option::is_none")]
    pub mustache_enabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchScriptTypeSettings {
    /// If enabled (default: true) then scripts are enabled, either for sandboxing languages (by default), or for all installed languages if 'sandbox_mode' is disabled (or for 6.x). NOTES: (Corresponds to the parameter 'script.file|stored/indexed|inline')
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// If enabled (default: true) and this script type is enabled, then only the sandbox languages are allowed. By default the sandbox languages are painless, expressions and mustache, but this can be restricted via the 'painless_enabled', 'mustache_enabled' 'expression_enabled' settings.NOTES: Not supported in 6.x. (Corresponds to the parameters 'script.engine.[painless|mustache|expressions].[file|stored|inline]')
    #[serde(rename = "sandbox_mode", skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchCuration {
    /// The destination instance configuration
    #[serde(rename = "to_instance_configuration_id")]
    pub to_instance_configuration_id: String,
    /// The source instance configuration
    #[serde(rename = "from_instance_configuration_id")]
    pub from_instance_configuration_id: String,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchUserBundle {
    /// The URL of the bundle, which must be accessible from the ECE infrastructure. This URL could be cached by platform, make sure to change it when updating the bundle
    #[serde(rename = "url")]
    pub url: String,
    /// The name of the bundle
    #[serde(rename = "name")]
    pub name: String,
    /// The supported Elasticsearch version (must match the version in the plan)
    #[serde(rename = "elasticsearch_version")]
    pub elasticsearch_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchUserPlugin {
    /// The URL of the plugin (must be accessible from the ECE infrastructure)
    #[serde(rename = "url")]
    pub url: String,
    /// The name of the plugin
    #[serde(rename = "name")]
    pub name: String,
    /// The supported Elasticsearch version (must match the version in the plan)
    #[serde(rename = "elasticsearch_version")]
    pub elasticsearch_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentTemplateReference {
    /// A version identifier to disambiguate multiple revisions of the same template
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The unique identifier of the deployment template
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchClusterSettings {
    #[serde(rename = "monitoring", skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<ManagedMonitoringSettings>,
    #[serde(rename = "curation", skip_serializing_if = "Option::is_none")]
    pub curation: Option<ClusterCurationSettings>,
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ClusterSnapshotSettings>,
    #[serde(rename = "traffic_filter", skip_serializing_if = "Option::is_none")]
    pub traffic_filter: Option<TrafficFilterSettings>,
    #[serde(rename = "trust", skip_serializing_if = "Option::is_none")]
    pub trust: Option<ElasticsearchClusterTrustSettings>,
    /// Threshold starting from which the number of instances in the cluster results in the introduction of dedicated masters. If the cluster is downscaled to a number of nodes below this one, dedicated masters will be removed. Limit is inclusive. When provided the threshold setting is updated. A `null` value removes the field. Otherwise, the setting remains as it was set previously.
    #[serde(rename = "dedicated_masters_threshold", skip_serializing_if = "Option::is_none")]
    pub dedicated_masters_threshold: Option<i32>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterMetadataSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedMonitoringSettings {
    /// The Id of the target cluster to which to send monitoring information
    #[serde(rename = "target_cluster_id")]
    pub target_cluster_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCurationSettings {
    /// Specifications for curation
    #[serde(rename = "specs")]
    pub specs: Vec<ClusterCurationSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCurationSpec {
    /// Index matching pattern
    #[serde(rename = "index_pattern")]
    pub index_pattern: String,
    /// Number of seconds after index creation to trigger this spec
    #[serde(rename = "trigger_interval_seconds")]
    pub trigger_interval_seconds: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSnapshotSettings {
    /// When set to true, the deployment will have SLM enabled. Default value is true.
    #[serde(rename = "slm", skip_serializing_if = "Option::is_none")]
    pub slm: Option<bool>,
    /// Cron expression indicating when should snapshots be taken. This can be enabled only if SLM is enabled for the deployment and 'interval' is not present
    #[serde(rename = "cron_expression", skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    /// Interval between snapshots, with the format 'length unit' (space is optional), where unit can be one of: d (day), h (hour), min (minute). Default is 30 minutes
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: Option<ClusterSnapshotRetention>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSnapshotRetention {
    /// Total retention period for all snapshots, with the format 'length unit' (space is optional), where unit can be one of: d (day), h (hour), min (minute)
    #[serde(rename = "max_age", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,
    /// Number of snapshots to retain
    #[serde(rename = "snapshots", skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchClusterTrustSettings {
    /// The list of trust relationships with different accounts
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountTrustRelationship>>,
    /// The list of trust relationships with external entities
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Vec<ExternalTrustRelationship>>,
    /// The list of trust relationships where the certificate is bundled with the trust setting. Allows configuring trust for clusters running outside of an Elastic Cloud managed environment or in an Elastic Cloud environment without an environment level trust established.
    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub direct: Option<Vec<DirectTrustRelationship>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectTrustRelationship {
    /// Auto generated identifier for this trust, allows distinguishing between update vs remove and add.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// a human readable name of the trust relationship
    #[serde(rename = "name")]
    pub name: String,
    /// A list of node names trusted in addition to those deducible from trust_allowlist and scope id. Allows trusting nodes that don't have a scoped name at the cost of maintaining the list. Mandatory if scope id is not defined. Wildcards are not allowed.
    #[serde(rename = "additional_node_names", skip_serializing_if = "Option::is_none")]
    pub additional_node_names: Option<Vec<String>>,
    /// A lowercase alphanumerical string of max 32 characters. Usually an organization id or an environment id, but could really be any suitable suffix for clusters using the CA certificate of this trust. Required unless trust_all is false and trust_allowlist is empty.
    #[serde(rename = "scope_id", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// If true, scope_id is required and the `trust_allowlist` is ignored and all clusters matching the scope id will be trusted.
    #[serde(rename = "trust_all")]
    pub trust_all: bool,
    /// The public ca certificate(s) to trust. Only one is required, but it is possible to specify multiple certificates in order to facilitate key rotation.
    #[serde(rename = "certificates")]
    pub certificates: Vec<TrustedCertificate>,
    /// The type can either be ESS, ECE or generic. If none is specified, then generic is assumed.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The list of clusters with matching scope to trust. Only used when `trust_all` is false. Providing one or more clusters makes scope_id mandatory.
    #[serde(rename = "trust_allowlist", skip_serializing_if = "Option::is_none")]
    pub trust_allowlist: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrustedCertificate {
    /// The public ca certificate as string in PEM format.
    #[serde(rename = "pem")]
    pub pem: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CertificateMetaData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateMetaData {
    /// The valid from date of the certificate in UTC
    #[serde(rename = "valid_from")]
    pub valid_from: String,
    /// Other deployments also trusting this certificate
    #[serde(rename = "also_trusted_by", skip_serializing_if = "Option::is_none")]
    pub also_trusted_by: Option<Vec<String>>,
    /// The expiry date of the certificate in UTC
    #[serde(rename = "valid_to")]
    pub valid_to: String,
    /// The fingerprint of the certificate
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
}

/// The type can either be ESS, ECE or generic. If none is specified, then generic is assumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ECE")]
    Ece,
    #[serde(rename = "ESS")]
    Ess,
    #[serde(rename = "generic")]
    Generic,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalTrustRelationship {
    /// The list of clusters to trust. Only used when `trust_all` is false.
    #[serde(rename = "trust_allowlist", skip_serializing_if = "Option::is_none")]
    pub trust_allowlist: Option<Vec<String>>,
    /// The ID of the external trust relationship
    #[serde(rename = "trust_relationship_id")]
    pub trust_relationship_id: String,
    /// The name of the external trust relationship. Retrieved from the TrustRelationship and ignored on write.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If true, all clusters in this external entity will be trusted and the `trust_allowlist` is ignored.
    #[serde(rename = "trust_all")]
    pub trust_all: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountTrustRelationship {
    /// If true, all clusters in this account will by default be trusted and the `trust_allowlist` is ignored.
    #[serde(rename = "trust_all")]
    pub trust_all: bool,
    /// the ID of the Account
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The list of clusters to trust. Only used when `trust_all` is false.
    #[serde(rename = "trust_allowlist", skip_serializing_if = "Option::is_none")]
    pub trust_allowlist: Option<Vec<String>>,
    /// A human readable name of the trust relationship
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchPayload {
    /// Alias to the Elasticsearch Cluster to attach Enterprise Search to
    pub elasticsearch_cluster_ref_id: String,
    /// The human readable name for the Enterprise Search cluster (default: takes the name of its Elasticsearch cluster)
    pub display_name: Option<String>,
    pub settings: Option<EnterpriseSearchSettings>,
    /// The region where this resource exists
    pub region: String,
    /// A locally-unique user-specified id for Enterprise Search
    pub ref_id: String,
    pub plan: EnterpriseSearchPlan,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchSettings {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterMetadataSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransientEnterpriseSearchPlanConfiguration {
    #[serde(rename = "plan_configuration", skip_serializing_if = "Option::is_none")]
    pub plan_configuration: Option<EnterpriseSearchPlanControlConfiguration>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<PlanStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchPlanControlConfiguration {
    /// Set to 'forced' to force a reboot as part of the upgrade plan
    #[serde(rename = "cluster_reboot", skip_serializing_if = "Option::is_none")]
    pub cluster_reboot: Option<ClusterReboot>,
    #[serde(rename = "move_allocators", skip_serializing_if = "Option::is_none")]
    pub move_allocators: Option<Vec<AllocatorMoveRequest>>,
    /// If true (default: false) does not allow re-using any existing instances currently in the cluster, i.e. even unchanged instances will be re-created
    #[serde(rename = "reallocate_instances", skip_serializing_if = "Option::is_none")]
    pub reallocate_instances: Option<bool>,
    /// List of allocators on which instances are placed if possible (if not possible/not specified then any available allocator with space is used)
    #[serde(rename = "preferred_allocators", skip_serializing_if = "Option::is_none")]
    pub preferred_allocators: Option<Vec<String>>,
    /// This timeout determines how long to give a cluster after it responds to API calls before performing actual operations on it. It defaults to 5s
    #[serde(rename = "calm_wait_time", skip_serializing_if = "Option::is_none")]
    pub calm_wait_time: Option<i64>,
    /// The total timeout in seconds after which the plan is cancelled even if it is not complete. Defaults to 4x the max memory capacity per node (in MB)
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// If true (default false), does not clear the maintenance flag (which prevents its API from being accessed except by the constructor) on new instances added until after a snapshot has been restored, otherwise, the maintenance flag is cleared once the new instances successfully join the new cluster
    #[serde(rename = "extended_maintenance", skip_serializing_if = "Option::is_none")]
    pub extended_maintenance: Option<bool>,
    #[serde(rename = "move_instances", skip_serializing_if = "Option::is_none")]
    pub move_instances: Option<Vec<InstanceMoveRequest>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceMoveRequest {
    /// An optional list of allocator ids to which the instance should be moved. If not specified then any available allocator can be used (including the current one if it is healthy)
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    /// The instance id that is going to be moved
    #[serde(rename = "from")]
    pub from: String,
    /// Tells the infrastructure that the instance should be considered as permanently down when deciding how to migrate data to new nodes. If left blank then the system will automatically decide (currently: will treat the instances as up)
    #[serde(rename = "instance_down", skip_serializing_if = "Option::is_none")]
    pub instance_down: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocatorMoveRequest {
    /// An optional list of allocator ids to which the instance(s) should be moved. If not specified then any available allocator can be used (including the current one if it is healthy)
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    /// The allocator id off which all instances in the cluster should be moved
    #[serde(rename = "from")]
    pub from: String,
    /// Tells the infrastructure that all instances on the allocator should be considered as permanently down when deciding how to migrate data to new nodes. If left blank then the system will auto-decide (currently: will treat the allocator as up)
    #[serde(rename = "allocator_down", skip_serializing_if = "Option::is_none")]
    pub allocator_down: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchPlan {
    #[serde(rename = "cluster_topology", skip_serializing_if = "Option::is_none")]
    pub cluster_topology: Option<Vec<EnterpriseSearchTopologyElement>>,
    #[serde(rename = "transient", skip_serializing_if = "Option::is_none")]
    pub transient: Option<TransientEnterpriseSearchPlanConfiguration>,
    #[serde(rename = "enterprise_search")]
    pub enterprise_search: EnterpriseSearchConfiguration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchTopologyElement {
    #[serde(rename = "node_count_per_zone", skip_serializing_if = "Option::is_none")]
    pub node_count_per_zone: Option<serde_json::Value>,
    /// number of zones in which nodes will be placed
    #[serde(rename = "zone_count", skip_serializing_if = "Option::is_none")]
    pub zone_count: Option<i32>,
    #[serde(rename = "node_configuration", skip_serializing_if = "Option::is_none")]
    pub node_configuration: Option<String>,
    #[serde(rename = "memory_per_node", skip_serializing_if = "Option::is_none")]
    pub memory_per_node: Option<serde_json::Value>,
    /// The version of the Instance Configuration Id. Unset for unversioned Instance Configurations on read. If unset in creates, means most recent version. If unset in updates, means keep the same version.
    #[serde(rename = "instance_configuration_version", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_version: Option<i32>,
    #[serde(rename = "enterprise_search", skip_serializing_if = "Option::is_none")]
    pub enterprise_search: Option<EnterpriseSearchConfiguration>,
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<EnterpriseSearchNodeTypes>,
    #[serde(rename = "allocator_filter", skip_serializing_if = "Option::is_none")]
    pub allocator_filter: Option<serde_json::Value>,
    /// Controls the allocation of this topology element as well as allowed sizes and node_types. It needs to match the id of an existing instance configuration.
    #[serde(rename = "instance_configuration_id", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_id: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<TopologySize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchConfiguration {
    /// An arbitrary JSON object allowing ECE admins to set clusters' parameters (only one of this and 'user_settings_override_yaml' is allowed), i.e. in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Enterprise Search settings)
    #[serde(rename = "user_settings_override_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_json: Option<serde_json::Value>,
    /// An arbitrary YAML object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_json' is allowed), provided the parameters are on the allowlist and not on the denylist. (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Enterprise Search settings)
    #[serde(rename = "user_settings_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_yaml: Option<String>,
    /// The version of the Enterprise Search cluster (must be one of the ECE supported versions, and won't work unless it matches the Elasticsearch version. Leave blank to auto-detect version.)
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// An arbitrary JSON object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_yaml' is allowed), provided the parameters are on the allowlist and not on the denylist. (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Enterprise Search settings)
    #[serde(rename = "user_settings_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_json: Option<serde_json::Value>,
    #[serde(rename = "system_settings", skip_serializing_if = "Option::is_none")]
    pub system_settings: Option<EnterpriseSearchSystemSettings>,
    /// An arbitrary YAML object allowing ECE admins to set clusters' parameters (only one of this and 'user_settings_override_json' is allowed), i.e. in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Enterprise Search settings)
    #[serde(rename = "user_settings_override_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_yaml: Option<String>,
    /// A docker URI that allows overriding of the default docker image specified for this version
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchSystemSettings {
    /// Optionally override the account within Enterprise Search - defaults to a system account that always exists (if specified, the username must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_password", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_password: Option<String>,
    /// Optionally override the account within Enterprise Search - defaults to a system account that always exists (if specified, the password must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_username", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_username: Option<String>,
    /// Optionally override the secret session key within Enterprise Search - defaults to the previously existing secretSession. Note that this field is never returned from the API, it is write only.
    #[serde(rename = "secret_session_key", skip_serializing_if = "Option::is_none")]
    pub secret_session_key: Option<String>,
    /// DEPRECATED: Scheduled for removal in a future version of the API.  Optionally override the URL to which to send data (for advanced users only, if unspecified the system selects an internal URL)
    #[serde(rename = "elasticsearch_url", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseSearchNodeTypes {
    /// Defines whether this instance should run as Connector
    #[serde(rename = "connector")]
    pub connector: bool,
    /// Defines whether this instance should run as Application/API server
    #[serde(rename = "appserver")]
    pub appserver: bool,
    /// Defines whether this instance should run as background worker
    #[serde(rename = "worker")]
    pub worker: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaPayload {
    /// Alias to the Elasticsearch Cluster to attach Kibana to
    #[serde(rename = "elasticsearch_cluster_ref_id")]
    pub elasticsearch_cluster_ref_id: String,
    /// The human readable name for the Kibana cluster (default: takes the name of its Elasticsearch cluster)
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<KibanaClusterSettings>,
    /// The region where this resource exists
    #[serde(rename = "region")]
    pub region: String,
    /// A locally-unique user-specified id for Kibana
    #[serde(rename = "ref_id")]
    pub ref_id: String,
    #[serde(rename = "plan")]
    pub plan: KibanaClusterPlan,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaClusterPlan {
    #[serde(rename = "cluster_topology", skip_serializing_if = "Option::is_none")]
    pub cluster_topology: Option<Vec<KibanaClusterTopologyElement>>,
    #[serde(rename = "transient", skip_serializing_if = "Option::is_none")]
    pub transient: Option<TransientKibanaPlanConfiguration>,
    #[serde(rename = "kibana")]
    pub kibana: KibanaConfiguration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransientKibanaPlanConfiguration {
    #[serde(rename = "plan_configuration", skip_serializing_if = "Option::is_none")]
    pub plan_configuration: Option<KibanaPlanControlConfiguration>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<PlanStrategy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaPlanControlConfiguration {
    /// Set to 'forced' to force a reboot as part of the upgrade plan
    #[serde(rename = "cluster_reboot", skip_serializing_if = "Option::is_none")]
    pub cluster_reboot: Option<ClusterReboot>,
    /// If true (default false), does not clear the maintenance flag (which prevents its API from being accessed except by the constructor) on new instances added until after a snapshot has been restored, otherwise, the maintenance flag is cleared once the new instances successfully join the new cluster
    #[serde(rename = "extended_maintenance", skip_serializing_if = "Option::is_none")]
    pub extended_maintenance: Option<bool>,
    /// This timeout determines how long to give a cluster after it responds to API calls before performing actual operations on it. It defaults to 5s
    #[serde(rename = "calm_wait_time", skip_serializing_if = "Option::is_none")]
    pub calm_wait_time: Option<i64>,
    /// The total timeout in seconds after which the plan is cancelled even if it is not complete. Defaults to 4x the max memory capacity per node (in MB)
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// Set to 'forced' to force a reboot as part of the upgrade plan
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClusterReboot {
    #[serde(rename = "forced")]
    Forced,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanStrategy {
    #[serde(rename = "rolling", skip_serializing_if = "Option::is_none")]
    pub rolling: Option<RollingStrategyConfig>,
    // below values are not defined in the OpenAPI spec
    // A strategy that creates new Elasticsearch instances, Kibana instances, and APM Servers with the new plan, then migrates the node data to minimize the amount of spare capacity.
//     #[serde(rename = "rolling_grow_and_shrink", skip_serializing_if = "Option::is_none")]
//     pub rolling_grow_and_shrink: Option<serde_json::Value>,
//     /// A strategy that creates instances with the new plan, migrates data from the old instances, then shuts down the old instances. `GrowShrinkStrategyConfig` is safer than 'rolling' and ensures single node availability during a plan change, but can be a lot slower on larger clusters.
//     #[serde(rename = "grow_and_shrink", skip_serializing_if = "Option::is_none")]
//     pub grow_and_shrink: Option<serde_json::Value>,
//     /// A strategy that lets constructor choose the most optimal way to execute the plan.
//     #[serde(rename = "autodetect", skip_serializing_if = "Option::is_none")]
//     pub autodetect: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RollingStrategyConfig {
    /// Whether to skip attempting to do a synced flush on the filesystem of the container (default: false), which is less safe but may be required if the container is unhealthy
    #[serde(rename = "skip_synced_flush", skip_serializing_if = "Option::is_none")]
    pub skip_synced_flush: Option<bool>,
    /// Whether we allow changing the capacity of instances (default false). This is currently implemented by stopping, re-creating then starting the affected instance on its associated allocator when performing the changes. NOTES: This requires a round-trip through the allocation infrastructure of the active constructor, as it has to reserve the target capacity without over-committing
    #[serde(rename = "allow_inline_resize", skip_serializing_if = "Option::is_none")]
    pub allow_inline_resize: Option<bool>,
    /// Specifies the grouping attribute to use when rolling several instances. Instances that share the same value for the provided attribute key are rolled together as a unit. Examples that make sense to use are '\\_\\_all\\_\\_' (roll all instances as a single unit), 'logical_zone_name' (roll instances by zone), '\\_\\_name\\_\\_' (roll one instance at a time, the default if not specified). Note that '\\_\\_all\\_\\_' is required when performing a major version upgrade
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// The time, in seconds, to wait for shards that show no progress of initializing before rolling the next group (default: 10 minutes)
    #[serde(rename = "shard_init_wait_time", skip_serializing_if = "Option::is_none")]
    pub shard_init_wait_time: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaClusterTopologyElement {
    /// The version of the Instance Configuration Id. Unset for unversioned Instance Configurations on read. If unset in creates, means most recent version. If unset in updates, means keep the same version.
    #[serde(rename = "instance_configuration_version", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_version: Option<i32>,
    /// Controls the allocation of this topology element as well as allowed sizes and node_types. It needs to match the id of an existing instance configuration.
    #[serde(rename = "instance_configuration_id", skip_serializing_if = "Option::is_none")]
    pub instance_configuration_id: Option<String>,
    /// number of zones in which nodes will be placed
    #[serde(rename = "zone_count", skip_serializing_if = "Option::is_none")]
    pub zone_count: Option<i32>,
    #[serde(rename = "kibana", skip_serializing_if = "Option::is_none")]
    pub kibana: Option<KibanaConfiguration>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<TopologySize>,
}


/// Type of resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Resource {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "storage")]
    Storage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaClusterSettings {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterMetadataSettings>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterMetadataSettings {
    /// The display name of the cluster
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaConfiguration {
    /// An arbitrary JSON object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_yaml' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Kibana settings)
    #[serde(rename = "user_settings_override_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_json: Option<serde_json::Value>,
    /// An arbitrary YAML object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_json' is allowed), provided the parameters are on the allowlist and not on the denylist. (These field together with 'user_settings_override*' and 'system_settings' defines the total set of Kibana settings)
    #[serde(rename = "user_settings_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_yaml: Option<String>,
    /// The version of the Kibana cluster (must be one of the ECE supported versions, and won't work unless it matches the Elasticsearch version. Leave blank to auto-detect version.)
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// An arbitrary JSON object allowing (non-admin) cluster owners to set their parameters (only one of this and 'user_settings_yaml' is allowed), provided the parameters are on the allowlist and not on the denylist. (This field together with 'user_settings_override*' and 'system_settings' defines the total set of Kibana settings)
    #[serde(rename = "user_settings_json", skip_serializing_if = "Option::is_none")]
    pub user_settings_json: Option<serde_json::Value>,
    #[serde(rename = "system_settings", skip_serializing_if = "Option::is_none")]
    pub system_settings: Option<KibanaSystemSettings>,
    /// An arbitrary YAML object allowing ECE admins owners to set clusters' parameters (only one of this and 'user_settings_override_json' is allowed), ie in addition to the documented 'system_settings'. (This field together with 'system_settings' and 'user_settings*' defines the total set of Kibana settings)
    #[serde(rename = "user_settings_override_yaml", skip_serializing_if = "Option::is_none")]
    pub user_settings_override_yaml: Option<String>,
    /// A docker URI that allows overriding of the default docker image specified for this version
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KibanaSystemSettings {
    /// Optionally override the account within Elasticsearch - defaults to a system account that always exists (if specified, the username must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_password", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_password: Option<String>,
    /// Optionally override the account within Elasticsearch - defaults to a system account that always exists (if specified, the password must also be specified). Note that this field is never returned from the API, it is write only.
    #[serde(rename = "elasticsearch_username", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_username: Option<String>,
    /// DEPRECATED: Scheduled for removal in a future version of the API.  Optionally override the URL to which to send data (for advanced users only, if unspecified the system selects an internal URL)
    #[serde(rename = "elasticsearch_url", skip_serializing_if = "Option::is_none")]
    pub elasticsearch_url: Option<String>,
}