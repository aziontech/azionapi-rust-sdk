/*
 * Network Lists API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkListResponseEntry {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "last_editor", skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<String>,
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "list_type", skip_serializing_if = "Option::is_none")]
    pub list_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "items_values", skip_serializing_if = "Option::is_none")]
    pub items_values: Option<Vec<String>>,
}

impl NetworkListResponseEntry {
    pub fn new() -> NetworkListResponseEntry {
        NetworkListResponseEntry {
            id: None,
            last_editor: None,
            last_modified: None,
            list_type: None,
            name: None,
            items_values: None,
        }
    }
}


