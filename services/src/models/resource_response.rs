/*
 * Services API
 *
 * Azion Services
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceResponse {
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "last_editor")]
    pub last_editor: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "trigger")]
    pub trigger: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ResourceResponse {
    pub fn new(content_type: String, id: i64, last_editor: String, name: String, trigger: String, updated_at: String) -> ResourceResponse {
        ResourceResponse {
            content_type,
            id,
            last_editor,
            name,
            trigger,
            updated_at,
        }
    }
}

