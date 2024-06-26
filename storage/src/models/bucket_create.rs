/*
 * Object Storage
 *
 * REST API OpenAPI documentation for the Object Storage
 *
 * The version of the OpenAPI document: 1.0.0 (v1)
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketCreate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "edge_access")]
    pub edge_access: models::EdgeAccessEnum,
}

impl BucketCreate {
    pub fn new(name: String, edge_access: models::EdgeAccessEnum) -> BucketCreate {
        BucketCreate {
            name,
            edge_access,
        }
    }
}

