/*
 * Digital Certificates API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dc406 {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
}

impl Dc406 {
    pub fn new() -> Dc406 {
        Dc406 {
            errors: None,
            schema_version: None,
        }
    }
}


