/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationCreateInstanceRequestArgs {
    Object(serde_json::Value),
    Array(Vec<serde_json::Value>),
}

impl Default for ApplicationCreateInstanceRequestArgs {
    fn default() -> Self {
        Self::Object(Default::default())
    }
}

