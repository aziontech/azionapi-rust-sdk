/*
 * Edge Node API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateServiceBindRequest {
    #[serde(rename = "variables")]
    pub variables: Vec<crate::models::Variable>,
}

impl UpdateServiceBindRequest {
    pub fn new(variables: Vec<crate::models::Variable>) -> UpdateServiceBindRequest {
        UpdateServiceBindRequest {
            variables,
        }
    }
}


