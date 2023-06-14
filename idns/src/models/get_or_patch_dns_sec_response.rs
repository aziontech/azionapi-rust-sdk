/*
 * Intelligent DNS
 *
 * Azion Intelligent DNS API
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetOrPatchDnsSecResponse : Object returned by get zone DNSSEC



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrPatchDnsSecResponse {
    /// The schema version
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Box<crate::models::DnsSec>>,
}

impl GetOrPatchDnsSecResponse {
    /// Object returned by get zone DNSSEC
    pub fn new() -> GetOrPatchDnsSecResponse {
        GetOrPatchDnsSecResponse {
            schema_version: None,
            results: None,
        }
    }
}

