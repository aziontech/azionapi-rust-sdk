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
pub struct CreateServiceRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateServiceRequest {
    pub fn new(name: String) -> CreateServiceRequest {
        CreateServiceRequest {
            name,
        }
    }
}


