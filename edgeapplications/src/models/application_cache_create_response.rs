/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCacheCreateResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Box<crate::models::ApplicationCacheCreateResults>>,
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
}

impl ApplicationCacheCreateResponse {
    pub fn new() -> ApplicationCacheCreateResponse {
        ApplicationCacheCreateResponse {
            results: None,
            schema_version: None,
        }
    }
}


