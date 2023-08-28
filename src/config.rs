use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub config: UserConfig,
    pub endpoints: HashMap<String, String>
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub project: String,
    pub key_path: String
}


