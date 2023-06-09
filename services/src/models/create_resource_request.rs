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
pub struct CreateResourceRequest {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "trigger")]
    pub trigger: String,
}

impl CreateResourceRequest {
    pub fn new(content: String, content_type: String, name: String, trigger: String) -> CreateResourceRequest {
        CreateResourceRequest {
            content,
            content_type,
            name,
            trigger,
        }
    }
}


