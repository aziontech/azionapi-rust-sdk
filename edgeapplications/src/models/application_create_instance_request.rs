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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCreateInstanceRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "edge_function_id")]
    pub edge_function_id: i64,
    #[serde(rename = "args")]
    pub args: Box<models::ApplicationCreateInstanceRequestArgs>,
}

impl ApplicationCreateInstanceRequest {
    pub fn new(name: String, edge_function_id: i64, args: models::ApplicationCreateInstanceRequestArgs) -> ApplicationCreateInstanceRequest {
        ApplicationCreateInstanceRequest {
            name,
            edge_function_id,
            args: Box::new(args),
        }
    }
}

