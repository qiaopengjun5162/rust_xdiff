use crate::RequestProfile;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use tokio::fs;

use super::{LoadConfig, ValidateConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, RequestProfile>,
}

impl LoadConfig for RequestConfig {}

impl RequestConfig {
    pub fn new(profiles: HashMap<String, RequestProfile>) -> Self {
        Self { profiles }
    }

    // pub async fn load_yaml(path: &str) -> Result<Self> {
    //     // todo: 如果文件不存在
    //     let content = fs::read_to_string(path).await?;
    //     Self::from_yaml(&content)
    // }

    // pub fn from_yaml(content: &str) -> Result<Self> {
    //     let config: Self = serde_yaml::from_str(content)?;
    //     config.validate()?;
    //     Ok(config)
    // }

    pub fn get_profile(&self, name: &str) -> Option<&RequestProfile> {
        self.profiles.get(name)
    }

    // pub fn validate(&self) -> Result<()> {
    //     for (name, profile) in &self.profiles {
    //         profile
    //             .validate()
    //             .context(format!("failed to validate profile: {}", name))?;
    //     }
    //     Ok(())
    // }
}

impl ValidateConfig for RequestConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .with_context(|| format!("profile: {}", name))?;
        }
        Ok(())
    }
}
