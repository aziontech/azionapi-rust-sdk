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
pub struct RuleSetResultResults {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "last_editor", skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<String>,
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "behaviors", skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<crate::models::Behaviors>>,
    #[serde(rename = "criteria", skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<Vec<crate::models::SslVerificationStatusCriteria>>>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

impl RuleSetResultResults {
    pub fn new() -> RuleSetResultResults {
        RuleSetResultResults {
            id: None,
            last_editor: None,
            last_modified: None,
            name: None,
            is_active: None,
            description: None,
            behaviors: None,
            criteria: None,
            order: None,
        }
    }
}

