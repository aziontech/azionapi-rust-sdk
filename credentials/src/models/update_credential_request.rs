/*
 * Credentials API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateCredentialRequest {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: bool,
}

impl UpdateCredentialRequest {
    pub fn new(description: String, name: String, status: bool) -> UpdateCredentialRequest {
        UpdateCredentialRequest {
            description,
            name,
            status,
        }
    }
}


