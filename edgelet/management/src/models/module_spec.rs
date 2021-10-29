/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSpec {
    /// The name of a the module.
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "config")]
    config: crate::models::Config,
    #[serde(rename = "imagePullPolicy", skip_serializing_if = "Option::is_none")]
    image_pull_policy: Option<String>,
}

impl ModuleSpec {
    pub fn new(name: String, type_: String, config: crate::models::Config) -> Self {
        ModuleSpec {
            name,
            type_,
            config,
            image_pull_policy: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_type(&mut self, type_: String) {
        self.type_ = type_;
    }

    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = type_;
        self
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn set_config(&mut self, config: crate::models::Config) {
        self.config = config;
    }

    pub fn with_config(mut self, config: crate::models::Config) -> Self {
        self.config = config;
        self
    }

    pub fn config(&self) -> &crate::models::Config {
        &self.config
    }

    pub fn set_image_pull_policy(&mut self, image_pull_policy: String) {
        self.image_pull_policy = Some(image_pull_policy);
    }

    pub fn with_image_pull_policy(mut self, image_pull_policy: String) -> Self {
        self.image_pull_policy = Some(image_pull_policy);
        self
    }

    pub fn image_pull_policy(&self) -> Option<&str> {
        self.image_pull_policy.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_image_pull_policy(&mut self) {
        self.image_pull_policy = None;
    }
}