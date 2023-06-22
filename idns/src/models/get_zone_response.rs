/*
 * Intelligent DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetZoneResponse : Object returned by get zone



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetZoneResponse {
    /// The schema version
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Box<crate::models::Zone>>,
}

impl GetZoneResponse {
    /// Object returned by get zone
    pub fn new() -> GetZoneResponse {
        GetZoneResponse {
            schema_version: None,
            results: None,
        }
    }
}


