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
pub struct RulesEngineBehavior {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<crate::models::RulesEngineBehaviorTarget>>,
}

impl RulesEngineBehavior {
    pub fn new(name: String) -> RulesEngineBehavior {
        RulesEngineBehavior {
            name,
            target: None,
        }
    }
}


