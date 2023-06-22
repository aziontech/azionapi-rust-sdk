/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationUpdateResponse {
    #[serde(rename = "results")]
    pub results: Box<crate::models::ApplicationUpdateResults>,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
}

impl ApplicationUpdateResponse {
    pub fn new(results: crate::models::ApplicationUpdateResults, schema_version: i64) -> ApplicationUpdateResponse {
        ApplicationUpdateResponse {
            results: Box::new(results),
            schema_version,
        }
    }
}


