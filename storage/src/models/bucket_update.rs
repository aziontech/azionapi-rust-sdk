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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketUpdate {
    #[serde(rename = "edge_access")]
    pub edge_access: models::EdgeAccessEnum,
}

impl BucketUpdate {
    pub fn new(edge_access: models::EdgeAccessEnum) -> BucketUpdate {
        BucketUpdate {
            edge_access,
        }
    }
}
