/*
 * Services API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateServiceRequest {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<crate::models::Variable>>,
}

impl UpdateServiceRequest {
    pub fn new() -> UpdateServiceRequest {
        UpdateServiceRequest {
            active: None,
            name: None,
            variables: None,
        }
    }
}


