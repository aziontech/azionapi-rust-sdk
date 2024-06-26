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
pub struct CreateApplicationResult {
    #[serde(rename = "results")]
    pub results: Box<models::ApplicationResultsCreate>,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
}

impl CreateApplicationResult {
    pub fn new(results: models::ApplicationResultsCreate, schema_version: i64) -> CreateApplicationResult {
        CreateApplicationResult {
            results: Box::new(results),
            schema_version,
        }
    }
}

