/*
 * Azion API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadRequestResponse {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    #[serde(rename = "items_values", skip_serializing_if = "Option::is_none")]
    pub items_values: Option<Vec<String>>,
    #[serde(rename = "list_type", skip_serializing_if = "Option::is_none")]
    pub list_type: Option<Vec<String>>,
}

impl BadRequestResponse {
    pub fn new() -> BadRequestResponse {
        BadRequestResponse {
            name: None,
            items_values: None,
            list_type: None,
        }
    }
}

