/*
 * Domain API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainResponseWithResult {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<models::DomainLinks>>,
    #[serde(rename = "results")]
    pub results: Box<models::DomainEntityResponse>,
    #[serde(rename = "total_pages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
}

impl DomainResponseWithResult {
    pub fn new(results: models::DomainEntityResponse, schema_version: i64) -> DomainResponseWithResult {
        DomainResponseWithResult {
            count: None,
            links: None,
            results: Box::new(results),
            total_pages: None,
            schema_version,
        }
    }
}

