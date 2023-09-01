use std::{fs::{read_to_string, File}, io::Write};
use anyhow::{Result, anyhow, Context};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub config: UserConfig,
    pub defaults: TypeConfig,
    pub statefull_override: Option<OptionalTypeConfig>,
    pub serverless_override: Option<OptionalTypeConfig>
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct UserConfig {
    /// sets the serverless project path
    pub project: String,
    pub key_path: String,
    pub default_deployment: String
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TypeConfig {
    pub url: String,
    pub base_path: String
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct OptionalTypeConfig {
    pub url: Option<String>,
    pub base_path: Option<String>,
    pub region: Option<String>
}

#[derive(Default, Clone)]
pub struct DeploymentSpecificConfig{
    pub url: String,
    pub base_path: String,
    pub region: Option<String>
}

impl From<TypeConfig> for DeploymentSpecificConfig {
    fn from(value: TypeConfig) -> Self {
        DeploymentSpecificConfig { 
            url: value.url, 
            base_path: value.base_path, 
            region: None }
    }
}

/// return a parsed config object
pub fn get_config() -> Result<Config> {
    let mut home = dirs::home_dir().ok_or_else(|| anyhow!("could not find home dir"))?;
    home.push(".config/ess/esscli.toml");

    let cfg_raw = read_to_string(&home).context(format!("error reading config file at {}", home.display()))?;
    let cfg: Config = toml::from_str(&cfg_raw).context("error reading config file")?;
    Ok(cfg)
}

/// create a new config object at the specified path
pub fn create_config(path: &str) -> Result<()> {
        let default = Config{
            config: UserConfig { 
                project: "observability".to_string(), 
                key_path: "~/.config/ess/api_key.txt".to_string(),
                default_deployment: "~/.config/ess/deployment.json".to_string()
            },
            defaults: TypeConfig { url: "https://console.qa.cld.elstc.co".to_string(), base_path: "/api/v1/".to_string() },
            statefull_override: None,
            serverless_override: Some(
                OptionalTypeConfig{
                    url: Some("https://global.qa.cld.elstc.co".to_string()), 
                    base_path: Some("/api/v1/serverless/".to_string()),
                    region: Some("aws-eu-west-1".to_string())
                }
            )
        };

        let full_create_path = shellexpand::tilde(&path);
        let out = toml::to_string_pretty(&default)?;
        let full_path = format!("{}/esscli.txt", full_create_path);
        let mut file = File::create(&full_path)?;
        write!(file, "{}", out)?;
        debug!("wrote default config file to {}", full_path);

        let deploy_path = format!("{}/deployment.json", full_create_path);
        let mut deploy_file = File::create(&deploy_path)?;
        write!(deploy_file, "{}", default_deploy_request())?;
        debug!("wrote default deployment request to {}", deploy_path);

        Ok(())
}


impl Config {
    /// resolve the endpoint config for serverless, taking into account overrides
    pub fn resolve_serverless(&self) -> DeploymentSpecificConfig {
        let mut found = DeploymentSpecificConfig::default();
        if let Some(overrides) = &self.serverless_override {
            found.url = overrides.url.as_ref().map_or_else(|| self.defaults.url.clone(), |v| v.clone());
            found.base_path = overrides.base_path.as_ref().map_or_else(|| self.defaults.base_path.clone(), |v|v.clone());
            found.region = overrides.region.clone()
        } else {
            found = self.defaults.clone().into()
        }

        found
    }

    /// resolve the endpoint config for stateful, taking into account overrides
    pub fn resolve_stateful(&self) -> DeploymentSpecificConfig{
        let mut found = DeploymentSpecificConfig::default();
        if let Some(overrides) = &self.statefull_override {
            found.url = overrides.url.as_ref().map_or_else(|| self.defaults.url.clone(), |v| v.clone());
            found.base_path = overrides.base_path.as_ref().map_or_else(|| self.defaults.base_path.clone(), |v|v.clone());
            found.region = overrides.region.clone()
        } else {
            found = self.defaults.clone().into()
        }
        found
    }
}

fn default_deploy_request() -> String {
    r#"{
        "resources": {
          "elasticsearch": [
            {
              "region": "gcp-us-central1", 
              "ref_id": "main-elasticsearch",
              "plan": {
                "cluster_topology": [
                  {
                    "zone_count": 2, 
                    "elasticsearch": {
                      "node_attributes": {
                        "data": "hot"
                      }
                    },
                    "instance_configuration_id": "gcp.es.datahot.n2.68x16x45", 
                    "node_roles": [
                      "master",
                      "ingest",
                      "transform",
                      "data_hot",
                      "remote_cluster_client",
                      "data_content"
                    ],
                    "id": "hot_content",
                    "size": {
                      "value": 4096, 
                      "resource": "memory"
                    }
                  },
                  {
                    "zone_count": 2,
                    "elasticsearch": {
                      "node_attributes": {
                        "data": "warm"
                      }
                    },
                    "instance_configuration_id": "gcp.es.datawarm.n2.68x10x190",
                    "node_roles": [
                      "data_warm",
                      "remote_cluster_client"
                    ],
                    "id": "warm",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  },
                  {
                    "zone_count": 1,
                    "elasticsearch": {
                      "node_attributes": {
                        "data": "cold"
                      }
                    },
                    "instance_configuration_id": "gcp.es.datacold.n2.68x10x190",
                    "node_roles": [
                      "data_cold",
                      "remote_cluster_client"
                    ],
                    "id": "cold",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  },
                  {
                    "zone_count": 1,
                    "elasticsearch": {
                      "node_attributes": {
                        "data": "frozen"
                      }
                    },
                    "instance_configuration_id": "gcp.es.datafrozen.n2.68x10x95",
                    "node_roles": [
                      "data_frozen"
                    ],
                    "id": "frozen",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  },
                  {
                    "zone_count": 3,
                    "instance_configuration_id": "gcp.es.master.n2.68x32x45",
                    "node_roles": [
                      "master",
                      "remote_cluster_client"
                    ],
                    "id": "master",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  },
                  {
                    "zone_count": 2,
                    "instance_configuration_id": "gcp.es.coordinating.n2.68x16x45",
                    "node_roles": [
                      "ingest",
                      "remote_cluster_client"
                    ],
                    "id": "coordinating",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  },
                  {
                    "zone_count": 1,
                    "instance_configuration_id": "gcp.es.ml.n2.68x32x45",
                    "node_roles": [
                      "ml",
                      "remote_cluster_client"
                    ],
                    "id": "ml",
                    "size": {
                      "resource": "memory",
                      "value": 0
                    }
                  }
                ],
                "elasticsearch": {
                  "version": "8.8.2",
                  "enabled_built_in_plugins": []
                },
                "deployment_template": {
                  "id": "gcp-general-purpose-v3" 
                }
              }
            }
          ],
          "kibana": [
            {
              "elasticsearch_cluster_ref_id": "main-elasticsearch",
              "region": "gcp-us-central1",
              "plan": {
                "cluster_topology": [
                  {
                    "instance_configuration_id": "gcp.kibana.n2.68x32x45",
                    "zone_count": 1, 
                    "size": {
                      "resource": "memory",
                      "value": 1024 
                    }
                  }
                ],
                "kibana": {
                  "version": "8.8.2"
                }
              },
              "ref_id": "main-kibana"
            }
          ],
          "integrations_server": [
            {
              "elasticsearch_cluster_ref_id": "main-elasticsearch",
              "region": "gcp-us-central1",
              "plan": {
                "cluster_topology": [
                  {
                    "instance_configuration_id": "gcp.integrationsserver.n2.68x32x45",
                    "zone_count": 1, 
                    "size": {
                      "resource": "memory",
                      "value": 1024 
                    }
                  }
                ],
                "integrations_server": {
                  "version": "8.8.2"
                }
              },
              "ref_id": "main-integrations_server"
            }
          ],
          "enterprise_search": [
            {
              "elasticsearch_cluster_ref_id": "main-elasticsearch",
              "region": "gcp-us-central1",
              "plan": {
                "cluster_topology": [
                  {
                    "node_type": {
                      "connector": true,
                      "appserver": true,
                      "worker": true
                    },
                    "instance_configuration_id": "gcp.enterprisesearch.n2.68x32x45",
                    "zone_count": 1, 
                    "size": {
                      "resource": "memory",
                      "value": 2048 
                    }
                  }
                ],
                "enterprise_search": {
                  "version": "8.8.2"
                }
              },
              "ref_id": "main-enterprise_search"
            }
          ]
        },
        "name": "my-first-api-deployment"
      }"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn test_resolve_stateful() {
        let cfg = Config{
            config: super::UserConfig::default(),
            defaults: super::TypeConfig { url: "default-url".to_string(), 
            base_path: "default_pat".to_string() },
            serverless_override: None,
            statefull_override: Some(super::OptionalTypeConfig { url: Some("stateful".to_string()), 
                base_path: Some("stateful_path".to_string()), 
                region: Some("stateful_region".to_string()) })
        };

        let res = cfg.resolve_stateful();
        assert_eq!(res.base_path, String::from("stateful_path"));
        assert_eq!(res.url, String::from("stateful"));
        assert_eq!(res.region, Some(String::from("stateful_region")));

    }

    #[test]
    fn test_resolve_serverless(){
        let cfg = Config{
            config: super::UserConfig::default(),
            defaults: super::TypeConfig { url: "default-url".to_string(), 
            base_path: "default_pat".to_string() },
            serverless_override: Some(super::OptionalTypeConfig { url: Some("serverless".to_string()), 
            base_path: Some("serverless_path".to_string()), 
            region: Some("serverless_region".to_string()) }),
            statefull_override: None
        };

        let res = cfg.resolve_serverless();
        assert_eq!(res.base_path, String::from("serverless_path"));
        assert_eq!(res.url, String::from("serverless"));
        assert_eq!(res.region, Some(String::from("serverless_region")));
    }
}