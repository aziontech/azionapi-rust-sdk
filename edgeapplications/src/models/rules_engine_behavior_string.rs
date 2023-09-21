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
pub struct RulesEngineBehaviorString {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "target")]
    pub target: String,
}

impl RulesEngineBehaviorString {
    pub fn new(name: String, target: String) -> RulesEngineBehaviorString {
        RulesEngineBehaviorString {
            name,
            target,
        }
    }
}

