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
pub struct CreateRulesEngineRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "criteria")]
    pub criteria: Vec<Vec<crate::models::RulesEngineCriteria>>,
    #[serde(rename = "behaviors")]
    pub behaviors: Vec<crate::models::RulesEngineBehavior>,
}

impl CreateRulesEngineRequest {
    pub fn new(name: String, criteria: Vec<Vec<crate::models::RulesEngineCriteria>>, behaviors: Vec<crate::models::RulesEngineBehavior>) -> CreateRulesEngineRequest {
        CreateRulesEngineRequest {
            name,
            description: None,
            criteria,
            behaviors,
        }
    }
}


