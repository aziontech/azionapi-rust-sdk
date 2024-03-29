/*
 * Intelligent DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostOrPutZoneResponse : Object returned by create or update zone



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostOrPutZoneResponse {
    /// The schema version
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
    /// The created hosted zone
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::Zone>>,
}

impl PostOrPutZoneResponse {
    /// Object returned by create or update zone
    pub fn new() -> PostOrPutZoneResponse {
        PostOrPutZoneResponse {
            schema_version: None,
            results: None,
        }
    }
}


