/*
 * Object Storage
 *
 * REST API OpenAPI documentation for the Object Storage
 *
 * The version of the OpenAPI document: 1.0.0 (v1)
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessObjectOperation {
    #[serde(rename = "state")]
    pub state: models::StateEnum,
    #[serde(rename = "data")]
    pub data: Box<models::ObjectResponseData>,
}

impl SuccessObjectOperation {
    pub fn new(state: models::StateEnum, data: models::ObjectResponseData) -> SuccessObjectOperation {
        SuccessObjectOperation {
            state,
            data: Box::new(data),
        }
    }
}

