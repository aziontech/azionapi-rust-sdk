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
pub struct ApplicationCachePutResponse {
    #[serde(rename = "results")]
    pub results: Box<crate::models::ApplicationCacheResponseDetails>,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
}

impl ApplicationCachePutResponse {
    pub fn new(results: crate::models::ApplicationCacheResponseDetails, schema_version: i64) -> ApplicationCachePutResponse {
        ApplicationCachePutResponse {
            results: Box::new(results),
            schema_version,
        }
    }
}


