/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RulesEngineIdResponse {
    #[serde(rename = "results")]
    pub results: Box<models::RulesEngineResultResponse>,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
}

impl RulesEngineIdResponse {
    pub fn new(results: models::RulesEngineResultResponse, schema_version: i64) -> RulesEngineIdResponse {
        RulesEngineIdResponse {
            results: Box::new(results),
            schema_version,
        }
    }
}

