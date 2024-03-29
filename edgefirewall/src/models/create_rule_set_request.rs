/*
 * Edge Firewall API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRuleSetRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "behaviors", skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<crate::models::Behaviors>>,
    #[serde(rename = "criteria", skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<Vec<crate::models::SslVerificationStatusCriteria>>>,
}

impl CreateRuleSetRequest {
    pub fn new() -> CreateRuleSetRequest {
        CreateRuleSetRequest {
            name: None,
            is_active: None,
            behaviors: None,
            criteria: None,
        }
    }
}


